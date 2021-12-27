pub mod servo;
use async_trait::async_trait;
use color_eyre::eyre::Result;

#[async_trait]
trait DigitalIo {
    async fn get(&self) -> Result<bool>;
    async fn set(&self, value: bool) -> Result<()>;
}

#[async_trait]
trait AnalogIo {
    async fn get(&self) -> Result<u8>;
    async fn set(&self, value: u8) -> Result<()>;
}
