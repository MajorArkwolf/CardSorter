use crate::{
    firmata_comm::FirmataComm,
    job_queue::{AsyncSerial, SerialHandle, SerialMutex},
    sensor::AnalogIo,
};
use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use std::sync::Arc;
use tokio::{sync::Mutex, task};

#[derive(Clone)]
pub struct Servo {
    board: Arc<Mutex<FirmataComm<AsyncSerial>>>,
    pin: i32,
    last_value: i32,
}

impl Servo {
    pub fn new(comm: FirmataComm<AsyncSerial>, pin: i32) -> Self {
        let board = Arc::new(Mutex::new(comm));
        Self {
            board,
            pin,
            last_value: 0,
        }
    }
}

#[async_trait]
impl AnalogIo for Servo {
    async fn get(&mut self) -> Result<i32> {
        Ok(self.last_value)
    }

    async fn set(&mut self, value: i32) -> Result<()> {
        self.last_value = value;
        let mut x = self.board.lock().await;
        task::block_in_place(|| {
            x.device
                .analog_write(self.pin, value)
                .wrap_err_with(|| "failed to write to board");
        });
        Ok(())
    }
}
