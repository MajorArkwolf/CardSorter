use async_channel::Receiver;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use tracing::debug;
use tracing::{event, instrument, Level};

use crate::subscriber::Publisher;

use super::IOSensor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Motor {
    id: u32,
    pin: firmata::PinId,
    #[serde(skip)]
    rx_array: Vec<Receiver<bool>>,
}

impl Motor {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Digital(pin);
        Self {
            id,
            pin,
            rx_array: vec![],
        }
    }

    async fn get<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<bool> {
        if board.get_physical_pin(self.pin)?.value > 0 {
            Ok(true)
        } else {
            Ok(false)
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

    fn publisher(&mut self) -> MotorPublisher {
        let (tx, rx) = async_channel::bounded::<bool>(20);
        self.rx_array.push(rx);

        MotorPublisher {
            publisher: Publisher::create(tx),
        }
    }
}

impl IOSensor for Motor {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        board
            .set_pin_mode(self.pin, firmata::OutputMode::ANALOG)
            .wrap_err_with(|| "failed to create servo")
    }
}

#[derive(Clone, Debug)]
pub struct MotorPublisher {
    publisher: Publisher<bool>,
}

impl MotorPublisher {
    #[instrument]
    pub async fn set(&mut self, value: bool) -> Result<()> {
        self.publisher.set(value).await
    }
}
