use crate::{arduino_board::ArduinoBoard, sensor::AnalogIo};
use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task};

#[derive(Clone, Debug)]
pub struct Servo<T: Read + Write + ?Sized> {
    board: ArduinoBoard<T>,
    pin: i32,
    last_value: i32,
}

impl<T: Read + Write + ?Sized> Servo<T> {
    pub async fn create(board: ArduinoBoard<T>, pin: i32) -> Result<Self> {
        board
            .board
            .lock()
            .await
            .set_pin_mode(pin, firmata::SERVO)
            .wrap_err_with(|| "failed to create servo")?;
        Ok(Self {
            board,
            pin,
            last_value: 0,
        })
    }
}

#[async_trait]
impl<T: Read + Write + Send + ?Sized> AnalogIo<T> for Servo<T> {
    async fn get(&mut self) -> Result<i32> {
        Ok(self.last_value)
    }

    async fn set(&mut self, value: i32) -> Result<()> {
        self.board
            .board
            .lock()
            .await
            .analog_write(self.pin, value)?;
        self.last_value = value;
        Ok(())
    }
}
