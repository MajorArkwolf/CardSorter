pub mod capture;
pub mod factory;
pub mod feeder;
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Report, Result};
use tokio::{sync::watch, task::JoinHandle};
use tracing::{debug, info};

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

pub struct Template {
    id: u32,
    type_of: Type,
}

pub struct CircuitWatcher {
    watch_tx: watch::Sender<State>,
    join_handles: Vec<JoinHandle<Result<()>>>,
}

impl CircuitWatcher {
    pub fn create(watch_tx: watch::Sender<State>) -> Self {
        Self {
            watch_tx,
            join_handles: vec![],
        }
    }

    pub fn add_join_handle(&mut self, joinhandle: JoinHandle<Result<()>>) {
        self.join_handles.push(joinhandle);
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Starting circuits.");
        self.watch_tx.send(State::Running)?;
        loop {}
        Ok(())
    }
}
