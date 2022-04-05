pub mod capture;
pub mod factory;
pub mod feeder;
use std::{sync::Arc, mem};

use futures::{stream::FuturesUnordered};
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result};
use tokio::{sync::{watch, Mutex}, task::JoinHandle};
use tokio_stream::StreamExt;
use tracing::{info, debug, error};

use crate::backbone::message::{OverseerChannel, Signal};

#[async_trait]
pub trait Circuit {
    fn get_id(&self) -> u32;
    fn get_state(&self) -> State;
    fn set_state(&mut self, state: State);
    async fn stop(&mut self) -> Result<()>;
    async fn setup(&mut self) -> Result<()>;
    async fn run(&mut self) -> Result<()>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Waiting,
    Stop,
    Running,
    Ending,
}

impl Default for State {
    fn default() -> Self {
        Self::Waiting
    }
}

#[derive(Debug)]
pub struct CircuitWatcher {
    current_state: Arc<Mutex<State>>,
    overseer_channel: OverseerChannel,
    watch_tx: watch::Sender<State>,
    sub_tasks: FuturesUnordered<JoinHandle<Result<()>>>,
}

impl Drop for CircuitWatcher {
    fn drop(&mut self) {
        drop(&self.overseer_channel);
        drop(&self.watch_tx);
        drop(&self.sub_tasks);
        drop(&self.current_state);
    }
}

impl CircuitWatcher {
    /**
     * All join handles must be registered before run otherwise they will not be added to the watcher.
     */
    pub fn add_join_handle(&mut self, joinhandle: JoinHandle<Result<()>>) {
        self.sub_tasks.push(joinhandle);
    }

    pub fn create(watch_tx: watch::Sender<State>, overseer_channel: OverseerChannel) -> Self {
        Self {
            current_state: Arc::new(Mutex::new(State::Waiting)),
            overseer_channel,
            watch_tx,
            sub_tasks: FuturesUnordered::new(),
        }
    }

    async fn process_tasks(sub_tasks: &mut FuturesUnordered<JoinHandle<Result<()>>>) -> State {
        let mut state = State::Ending;
        while let Some(item) = sub_tasks.next().await {
            match item {
                Ok(res) => { 
                    match res {
                        Ok(()) => {
                            debug!("Task returned ending.");
                        },
                        Err(err) => {
                            error!("Task returned error: {}", err);
                            state = State::Stop;
                            break;
                        },
                    }
                },
                Err(err) => {
                    error!("Task join error: {}", err);
                    state = State::Stop;
                    break;
                },
            }
        }

        return state;
    }

    fn should_stop(state: State) -> bool {
        match state {
            State::Waiting | State::Running => false,
            State::Stop | State::Ending => true,
        }
    }

    async fn compare_global_state(global_state: &mut Arc<Mutex<State>>, state: State) -> bool {
        let curr_state = global_state.lock().await;
        return *curr_state == state;
    }

    async fn update_global_state(global_state: &mut Arc<Mutex<State>>, new_state: State) -> bool {
        let mut curr_state = global_state.lock().await;
        if *curr_state == State::Stop || *curr_state == State::Ending {
            return false;
        } else {
            *curr_state = new_state;
            return true;
        }
    }

    pub fn kill_all_tasks(tasks: &mut FuturesUnordered<JoinHandle<Result<()>>>) {
        debug!("kill all tasks called");
        tasks.clear();
    }

    fn start_background_task(global_state: Arc<Mutex<State>>, tasks: FuturesUnordered<JoinHandle<Result<()>>>) -> JoinHandle<Result<()>> {
        tokio::task::spawn(async move {
            let mut global_state = global_state;
            let mut old_state = global_state.lock().await.clone();
            let mut tasks = tasks;
            

            loop {
                let result = CircuitWatcher::process_tasks(&mut tasks).await;

                if !CircuitWatcher::compare_global_state(&mut global_state, old_state).await {
                    old_state = global_state.lock().await.clone();
                }

                if !CircuitWatcher::should_stop(old_state) && CircuitWatcher::should_stop(result) {
                    if !CircuitWatcher::update_global_state(&mut global_state, result).await {
                        return Err(eyre!("unable to update global state during a stop."));
                    }
                    else {
                        break;
                    }
                } 
                
                if CircuitWatcher::should_stop(old_state) || CircuitWatcher::should_stop(result)
                {
                    debug!("background task recieved a stop/end state.");
                    break;
                }
            }
            info!("background task returning");
            return Ok(());
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting run method.");
        let mut old_state = self.current_state.lock().await.clone();
        self.watch_tx.send(old_state)?;

        let mut new_tasks:FuturesUnordered<JoinHandle<Result<()>>> = FuturesUnordered::new();
        mem::swap(&mut new_tasks, &mut self.sub_tasks);
    
        self.sub_tasks.push(CircuitWatcher::start_background_task(self.current_state.clone(), new_tasks));

        info!("Began task loop, waiting responses.");
        while !CircuitWatcher::should_stop(old_state) {
            let mut state = old_state;

            if let Ok(item) = self.overseer_channel.rx.try_recv() {
                match item.signal {
                    Signal::Kill => todo!(),
                    Signal::Stop => {
                        info!("stop command recieved from overseer.");
                        state = State::Stop;
                    },
                    Signal::Start | Signal::Resume => {
                        if CircuitWatcher::should_stop(state) {
                            info!("Start request ignored since circuit watcher is stopping/ending.");
                        } else {
                            state = State::Running;
                        }
                    },
                    Signal::Pause => {
                        if CircuitWatcher::should_stop(state) {
                            info!("Pause request ignore since circuit watcher is stopping/ending.");
                        } else {
                            state = State::Waiting;
                        }
                    },
                    Signal::HeartBeat => {},
                    Signal::Ack => {},
                }
                self.overseer_channel.acknowledge().await?
            }
            if CircuitWatcher::compare_global_state(&mut self.current_state, old_state).await {
                if !CircuitWatcher::update_global_state(&mut self.current_state, state).await {
                    debug!("unable to update the global state, exiting main loop");
                    break;
                } else {
                    self.watch_tx.send(state)?;
                    old_state = state;
                }
            } else {
                let new_global_state = self.current_state.lock().await.clone();
                match new_global_state {
                    State::Waiting => {
                        match state {
                            State::Waiting => {},
                            State::Stop | State::Running | State::Ending => {
                                CircuitWatcher::update_global_state(&mut self.current_state, state).await;
                                break;
                            },
                        }
                    },
                    State::Stop => {break;},
                    State::Running => {CircuitWatcher::update_global_state(&mut self.current_state, state).await;},
                    State::Ending => {break;},
                }
            }
        }

        let state = self.current_state.lock().await.clone();

        info!("Task loop has been broken ({:?}), beginning clean up.", state);
        match state {
            State::Waiting | State::Running => {},
            State::Stop => {
                self.watch_tx.send(state)?;
            },
            State::Ending => {
                self.watch_tx.send(state)?;
                while !self.sub_tasks.is_empty() {}
                debug!("Background task rejoined.");
            },
        }
        self.sub_tasks.clear();
        debug!("Watcher task ran to completion.");
        Ok(())
    }
}
