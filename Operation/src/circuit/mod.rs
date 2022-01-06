use color_eyre::eyre::Result;
pub mod capture;
pub mod circuit_constructor;
pub mod circuit_controller;
pub mod feeder;
use async_trait::async_trait;
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CircuitState {
    Ready,
    Running,
    Waiting,
    Stopped,
}

#[async_trait]
pub trait Circuit {
    async fn get_id(&self) -> u32;
    async fn get_state(&self) -> CircuitState;
    async fn change_state(&mut self, next_state: CircuitState) -> Result<()>;
    async fn update(&mut self);
    async fn stop(&mut self);
}
