pub mod capture;
pub mod factory;
pub mod feeder;
use futures::{stream::FuturesUnordered};
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result};
use tokio::{sync::watch, task::JoinHandle};
use tokio_stream::StreamExt;
use tracing::{info, debug, error};

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

pub enum Type {
    Feeder,
    Distributor,
    Dispensor,
}

pub struct CircuitWatcher {
    current_state: State,
    watch_tx: watch::Sender<State>,
    sub_tasks: FuturesUnordered<JoinHandle<Result<()>>>,
}

impl CircuitWatcher {
    pub fn add_join_handle(&mut self, joinhandle: JoinHandle<Result<()>>) {
        self.sub_tasks.push(joinhandle);
    }

    pub fn create(watch_tx: watch::Sender<State>) -> Self {
        Self {
            current_state: State::Waiting,
            watch_tx,
            sub_tasks: FuturesUnordered::new(),
        }
    }

    async fn process_tasks(&mut self) -> State {
        while let Some(item) = self.sub_tasks.next().await {
            match item {
                Ok(res) => { 
                    match res {
                        Ok(()) => {
                            debug!("Task returned ending.");
                            return State::Ending;
                        },
                        Err(err) => {
                            error!("Task returned error: {}", err);
                            return State::Stop;
                        },
                    }
                },
                Err(err) => {
                    error!("Task join error: {}", err);
                    return State::Stop;
                },
            }
        }
        debug!("Fell through, returning waiting.");
        return State::Waiting;
    }

    fn should_stop(state: State) -> bool {
        match state {
            State::Waiting | State::Running => false,
            State::Stop | State::Ending => true,
        }
    }

    pub fn kill_all_tasks(&mut self) {
        debug!("kill all tasks called");
        self.sub_tasks.clear();
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting circuits.");
        self.current_state = State::Running;
        self.watch_tx.send(self.current_state)?;

        info!("Began watching tasks.");
        while CircuitWatcher::should_stop(self.current_state) {
            let result = self.process_tasks().await;
            if self.current_state != State::Stop {
                self.current_state = result;
            }
            self.watch_tx.send(self.current_state)?;
        }

        info!("Task loop has been broken, beginning clean up.");
        match self.current_state {
            State::Waiting | State::Running => {},
            State::Stop => {
                self.watch_tx.send(self.current_state)?;
                self.kill_all_tasks();
                return Err(eyre!("A stop state was recieved from a task, this should not occur."));
            },
            State::Ending => {
                self.watch_tx.send(self.current_state)?;
                loop {
                    let result = self.process_tasks().await;
                    match result {
                        State::Running | State::Stop => {
                            error!("While attempting to resolve a ending state, a {:?} state was returned. This is undefined, terminating tasks", result);
                            self.kill_all_tasks();
                            return Err(eyre!("A stop state was returned when trying to resolve an end state."));
                        },
                        State::Ending => continue, // Keep looping until a waiting is returned.
                        State::Waiting => break, // Waiting is the fall through statement that occures when no tasks are left.
                    }
                }
            },
        }
        debug!("Run task ran to completion.");
        Ok(())
    }
}
