use crate::subscriber;
use async_channel::{Receiver, Sender};
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use subscriber::{Publisher, Subscriber};
use tracing::error;
use tracing::instrument;

use super::IOSensor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Servo {
    id: u32,
    pin: firmata::PinId,
    #[serde(skip)]
    value: i32,
    #[serde(skip)]
    tx_array: Vec<Sender<i32>>,
    #[serde(skip)]
    rx_array: Vec<Receiver<i32>>,
}

impl Servo {
    pub fn create(id: u32, pin: u8) -> Self {
        let pin = firmata::PinId::Pin(pin);

        Self {
            id,
            pin,
            value: 0,
            tx_array: vec![],
            rx_array: vec![],
        }
    }

    pub async fn get<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<i32> {
        Ok(board.get_physical_pin(self.pin)?.value)
    }

    pub fn publisher(&mut self) -> ServoPublisher {
        let (tx, rx) = async_channel::bounded::<i32>(20);
        self.rx_array.push(rx);

        ServoPublisher {
            publisher: Publisher::create(tx),
        }
    }

    pub async fn set<T: Read + Write>(
        &mut self,
        board: &mut firmata::Board<T>,
        value: i32,
    ) -> Result<()> {
        board.analog_write(self.pin, value)?;
        Ok(())
    }

    pub fn subscribe(&mut self) -> ServoSubscriber {
        let (tx, rx) = async_channel::bounded::<i32>(20);
        self.tx_array.push(tx);

        ServoSubscriber {
            subscriber: Subscriber::create(rx),
        }
    }

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        for channel in self.rx_array.iter() {
            if !channel.is_empty() && !channel.is_closed() {
                let value = match channel.recv().await {
                    Ok(v) => v,
                    Err(_) => {
                        error!("recv error when listening to publishers");
                        continue;
                    }
                };
                board.analog_write(self.pin, value)?;
            }
        }

        let new_value = self.get(board).await?;
        if new_value != self.value {
            self.broadcast_to_subscribers(new_value).await?;
            self.value = new_value;
        }

        Ok(())
    }

    #[instrument]
    async fn broadcast_to_subscribers(&mut self, value: i32) -> Result<()> {
        for comm in self.tx_array.iter() {
            if !comm.is_full() && !comm.is_closed() {
                match comm.send(value).await {
                    Ok(_) => continue,
                    Err(_) => {
                        error!(
                            "failed to send update to subscriber from photoresistor {}",
                            self.id
                        );
                        comm.close();
                    }
                }
            }
        }
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

#[derive(Clone, Debug)]
pub struct ServoPublisher {
    publisher: Publisher<i32>,
}

impl ServoPublisher {
    #[instrument]
    pub async fn set(&mut self, value: i32) -> Result<()> {
        self.publisher.set(value).await
    }
}

#[derive(Clone, Debug)]
pub struct ServoSubscriber {
    subscriber: Subscriber<i32>,
}

impl ServoSubscriber {
    #[instrument]
    pub async fn get(&mut self) -> Result<i32> {
        self.subscriber.get().await
    }
}
