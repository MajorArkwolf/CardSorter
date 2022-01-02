use crate::subscriber;
use async_channel::Sender;
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::Arc,
};
use subscriber::Subscriber;
use tokio::{sync::Mutex, task};
use tracing::{debug, error, info};

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

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        let new_value = self.get(board).await?;
        if new_value != self.value {
            for comm in self.tx_array.iter() {
                match comm.send(new_value).await {
                    Ok(_) => continue,
                    Err(_) => info!(
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

    pub async fn subscribe(&mut self) -> PhotoResistorSubscriber {
        let (tx, rx) = async_channel::bounded::<i32>(1);
        self.tx_array.push(tx);

        PhotoResistorSubscriber {
            subscriber: Subscriber::create(rx, self.value),
        }
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
            .wrap_err_with(|| "failed to create photoresistor")?;
        board
            .report_analog(self.pin, true)
            .wrap_err_with(|| "failed to register photoresistor for updates")
    }
}

pub struct PhotoResistorSubscriber {
    subscriber: Subscriber<i32>,
}

impl PhotoResistorSubscriber {
    pub async fn get(&mut self) -> Result<i32> {
        self.subscriber.get().await
    }
}
