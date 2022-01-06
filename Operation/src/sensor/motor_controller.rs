use crate::subscriber;
use async_channel::Receiver;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use subscriber::Publisher;
use tracing::debug;
use tracing::{event, instrument, Level};

use super::IOSensor;

const MOTOR_A_PINS_INDEX: [usize; 2] = [0, 1];
const MOTOR_B_PINS_INDEX: [usize; 2] = [2, 3];
const MOTOR_FORWARD: [i32; 2] = [1, 0];
const MOTOR_REVERSE: [i32; 2] = [0, 1];
const MOTOR_STOP: [i32; 2] = [0, 0];

#[derive(Clone, Debug, Copy)]
pub enum Motor {
    A,
    B,
}
#[derive(Clone, Debug, Copy)]
pub enum Movement {
    Stop,
    Forward,
    Reverse,
}

impl Default for Movement {
    fn default() -> Self {
        Movement::Stop
    }
}
#[derive(Clone, Debug, Copy)]
pub struct MotorControllerMessage {
    motor: Motor,
    movement: Movement,
}

impl MotorControllerMessage {
    pub fn create(motor: Motor, movement: Movement) -> Self {
        Self { motor, movement }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MotorController {
    id: u32,
    pins: [u8; 4],
    #[serde(skip)]
    rx_array: Vec<Receiver<MotorControllerMessage>>,
}

impl MotorController {
    pub fn create(id: u32, pins: [u8; 4]) -> Self {
        Self {
            id,
            pins,
            rx_array: vec![],
        }
    }

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        let mut result: Option<MotorControllerMessage> = None;
        for channel in self.rx_array.iter_mut() {
            if !channel.is_empty() && !channel.is_closed() {
                match channel.recv().await {
                    Ok(v) => {
                        result = Some(v);
                        break;
                    }
                    Err(_) => channel.close(),
                };
            }
        }

        if let Some(v) = result {
            self.set_motor(board, v.motor, v.movement).await?
        }

        Ok(())
    }

    pub async fn set_motor<T: Read + Write>(
        &self,
        board: &mut firmata::Board<T>,
        motor: Motor,
        movement: Movement,
    ) -> Result<()> {
        let pins = match motor {
            Motor::A => [
                self.pins[MOTOR_A_PINS_INDEX[0]],
                self.pins[MOTOR_A_PINS_INDEX[1]],
            ],
            Motor::B => [
                self.pins[MOTOR_B_PINS_INDEX[0]],
                self.pins[MOTOR_B_PINS_INDEX[1]],
            ],
        };
        if pins[0] == 0 && pins[1] == 0 {
            return Err(eyre!("Pin {} and {} are not valid pins", pins[0], pins[1]));
        }
        let digital_assignment = match movement {
            Movement::Stop => MOTOR_STOP,
            Movement::Forward => MOTOR_FORWARD,
            Movement::Reverse => MOTOR_REVERSE,
        };
        for (i, pin) in pins.iter().enumerate() {
            board.digital_write(firmata::PinId::Digital(*pin), digital_assignment[i])?;
        }
        Ok(())
    }

    pub fn publisher(&mut self) -> MotorControllerPublisher {
        let (tx, rx) = async_channel::bounded::<MotorControllerMessage>(20);
        self.rx_array.push(rx);

        MotorControllerPublisher {
            publisher: Publisher::create(tx),
        }
    }
}

impl IOSensor for MotorController {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        for pin in self.pins {
            board
                .set_pin_mode(firmata::PinId::Digital(pin), firmata::OutputMode::OUTPUT)
                .wrap_err_with(|| "failed to create motor controller")?;
        }
        board.digital_write(firmata::PinId::Digital(4), 1)?;
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct MotorControllerPublisher {
    publisher: Publisher<MotorControllerMessage>,
}

impl MotorControllerPublisher {
    #[instrument]
    pub async fn set(&mut self, value: MotorControllerMessage) -> Result<()> {
        self.publisher.set(value).await
    }
}
