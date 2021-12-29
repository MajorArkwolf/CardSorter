use crate::{arduino_board::ArduinoBoard, sensor::DigitalIo};
use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task};

#[derive(Clone, Debug)]
pub struct Motor<T: Read + Write + ?Sized> {
    board: ArduinoBoard<T>,
    pin: i32,
    last_value: bool,
}

impl<T: Read + Write + ?Sized> Motor<T> {
    pub async fn create(board: ArduinoBoard<T>, pin: i32) -> Result<Self> {
        board
            .board
            .lock()
            .await
            .set_pin_mode(pin, firmata::OUTPUT)
            .wrap_err_with(|| "failed to set pin mode")?;
        Ok(Self {
            board,
            pin,
            last_value: false,
        })
    }
}

#[async_trait]
impl<T: Read + Write + Send + ?Sized> DigitalIo<T> for Motor<T> {
    async fn get(&mut self) -> Result<bool> {
        Ok(self.last_value)
    }

    async fn set(&mut self, value: bool) -> Result<()> {
        self.board
            .board
            .lock()
            .await
            .digital_write(self.pin, value as i32)?;
        self.last_value = value;
        Ok(())
    }
}
