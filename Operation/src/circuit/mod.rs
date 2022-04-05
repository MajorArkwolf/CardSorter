pub mod capture;
pub mod factory;
pub mod feeder;
pub mod circuitwatcher;

use async_trait::async_trait;
use color_eyre::eyre::{Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Waiting,
    Stop,
    Running,
    Ending,
}

#[async_trait]
pub trait Circuit {
    fn get_id(&self) -> u32;
    fn get_state(&self) -> State;
    fn set_state(&mut self, state: State);
    async fn stop(&mut self) -> Result<()>;
    async fn setup(&mut self) -> Result<()>;
    async fn run(&mut self) -> Result<()>;
}
