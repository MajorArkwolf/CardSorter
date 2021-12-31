use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task};

use super::IOSensor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Servo {
    id: u32,
    pin: firmata::PinId,
}

impl Servo {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Pin(pin);
        Self { id, pin }
    }

    pub async fn get(&mut self) -> Result<i32> {
        Ok(0)
    }

    pub async fn set<T: Read + Write>(
        &mut self,
        board: &mut firmata::Board<T>,
        value: i32,
    ) -> Result<()> {
        board.analog_write(self.pin, value)?;
        Ok(())
    }
}

impl IOSensor for Servo {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        board
            .set_pin_mode(self.pin, firmata::OutputMode::SERVO)
            .wrap_err_with(|| "failed to create servo")
    }
}
