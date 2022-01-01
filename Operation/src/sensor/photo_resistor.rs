use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task};
use tracing::{debug, error, info};

use super::IOSensor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotoResistor {
    id: u32,
    pin: firmata::PinId,
}

impl PhotoResistor {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Analog(pin);
        Self { id, pin }
    }

    async fn get<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<i32> {
        println!(
            "Servo: {}, Analog: {}",
            board.get_physical_pin(firmata::PinId::Pin(12))?.value,
            board.get_physical_pin(self.pin)?.value
        );
        Ok(board.get_physical_pin(self.pin)?.value)
    }

    async fn set(&mut self, value: i32) -> Result<()> {
        Err(eyre!("not implemented"))
    }
}

impl IOSensor for PhotoResistor {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        debug!("registering photo_resistor: {:?}", self);
        board
            .set_pin_mode(self.pin, firmata::OutputMode::ANALOG)
            .wrap_err_with(|| "failed to create servo")
    }
}
