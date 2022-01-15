use crate::subscriber;
use async_channel::Sender;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use subscriber::Subscriber;
use tracing::{debug, error, info};
use tracing::{event, instrument, Level};

use super::IOSensor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotoResistor {
    id: u32,
    pin: firmata::PinId,
    #[serde(skip)]
    value: i32,
    #[serde(skip)]
    tx_array: Vec<Sender<i32>>,
}

impl PhotoResistor {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Analog(pin);
        Self {
            id,
            pin,
            value: 0,
            tx_array: vec![],
        }
    }

    pub fn get_pin_id(&self) -> firmata::PinId {
        self.pin
    }

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        let new_value = self.get(board).await?;
        if new_value != self.value {
            self.broadcast_to_subscribers(new_value).await?;
        }
        Ok(())
    }

    #[instrument]
    async fn broadcast_to_subscribers(&mut self, value: i32) -> Result<()> {
        for comm in self.tx_array.iter() {
            if !comm.is_full() && !comm.is_closed() {
                match comm.send(value).await {
                    Ok(_) => continue,
                    Err(_) => error!(
                        "failed to send update to subscriber from photoresistor {}",
                        self.id
                    ),
                }
            }
        }
        Ok(())
    }

    pub async fn get<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<i32> {
        let val = board.get_physical_pin(self.pin)?.value;
        Ok(val)
    }

    pub fn subscribe(&mut self) -> PhotoResistorSubscriber {
        let (tx, rx) = async_channel::bounded::<i32>(20);
        self.tx_array.push(tx);

        PhotoResistorSubscriber {
            subscriber: Subscriber::create(rx),
        }
    }
}

impl IOSensor for PhotoResistor {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        board
            .set_pin_mode(self.pin, firmata::OutputMode::ANALOG)
            .wrap_err_with(|| "failed to create photoresistor")?;
        board
            .report_analog(self.pin, true)
            .wrap_err_with(|| "failed to register photoresistor for updates")
    }
}
#[derive(Clone, Debug)]
pub struct PhotoResistorSubscriber {
    subscriber: Subscriber<i32>,
}

impl PhotoResistorSubscriber {
    pub async fn get(&mut self) -> Result<i32> {
        self.subscriber.get().await
    }
}
