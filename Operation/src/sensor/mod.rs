pub mod servo;
use async_trait::async_trait;
use color_eyre::eyre::Result;

#[async_trait]
pub trait DigitalIo {
    async fn get(&mut self) -> Result<bool>;
    async fn set(&mut self, value: bool) -> Result<()>;
}

#[async_trait]
pub trait AnalogIo {
    async fn get(&mut self) -> Result<i32>;
    async fn set(&mut self, value: i32) -> Result<()>;
}
