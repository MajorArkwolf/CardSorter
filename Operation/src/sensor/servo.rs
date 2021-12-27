use crate::{
    job_queue::{AsyncSerial, SerialHandle, SerialMutex},
    sensor::AnalogIo,
};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Servo {
    port: SerialMutex,
    pin: u8,
}

impl Servo {
    pub async fn new(port: SerialHandle, pin: u8) -> Self {
        let port = Arc::new(Mutex::new(port));
        Self { port, pin }
    }
}

#[async_trait]
impl AnalogIo for Servo {
    async fn get(&self) -> Result<u8> {
        //serial.digital_read(13);
        todo!()
    }

    async fn set(&self, value: u8) -> Result<()> {
        //serial.digital_write(13, i);
        todo!()
    }
}
