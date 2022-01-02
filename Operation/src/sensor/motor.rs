use color_eyre::eyre::{Result, WrapErr};
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
pub struct Motor {
    id: u32,
    pin: firmata::PinId,
}

impl Motor {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Digital(pin);
        Self { id, pin }
    }

    async fn get<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<bool> {
        if board.get_physical_pin(self.pin)?.value > 0 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    async fn set<T: Read + Write>(
        &mut self,
        board: &mut firmata::Board<T>,
        value: bool,
    ) -> Result<()> {
        board.digital_write(self.pin, value as i32)?;
        Ok(())
    }
}

impl IOSensor for Motor {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        debug!("registering motor: {:?}", self);
        board
            .set_pin_mode(self.pin, firmata::OutputMode::ANALOG)
            .wrap_err_with(|| "failed to create servo")
    }
}
