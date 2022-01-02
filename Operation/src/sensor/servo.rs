use crate::subscriber;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use subscriber::{Publisher, Subscriber};
use tracing::{debug, error, info};

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
            subscriber: Subscriber::create(rx, self.value),
        }
    }

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        for channel in self.rx_array.iter_mut() {
            let value = match channel.try_recv() {
                Ok(v) => v,
                Err(e) => match e {
                    async_channel::TryRecvError::Empty => continue,

                    async_channel::TryRecvError::Closed => {
                        debug!("servo publisher closed the channel.");
                        continue;
                    }
                },
            };
            board.analog_write(self.pin, value)?;
        }

        let new_value = self.get(board).await?;
        if new_value != self.value {
            for channel in self.tx_array.iter_mut() {
                match channel.try_send(new_value) {
                    Ok(_) => continue,
                    Err(e) => match e {
                        async_channel::TrySendError::Full(_) => {
                            debug!("queue is full to servo subscriber")
                        }
                        async_channel::TrySendError::Closed(_) => {
                            debug!("subscriber has closed their connection")
                        }
                    },
                }
            }
            self.value = new_value;
        }

        Ok(())
    }
}

impl IOSensor for Servo {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        debug!("registering servo: {:?}", self);
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
    pub async fn set(&mut self, value: i32) -> Result<()> {
        self.publisher.set(value).await
    }
}

#[derive(Clone, Debug)]
pub struct ServoSubscriber {
    subscriber: Subscriber<i32>,
}

impl ServoSubscriber {
    pub async fn get(&mut self) -> Result<i32> {
        self.subscriber.get().await
    }
}
