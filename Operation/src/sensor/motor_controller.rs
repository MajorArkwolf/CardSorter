use crate::subscriber;
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::Arc,
};
use subscriber::{Publisher, Subscriber};
use tokio::{sync::Mutex, task};
use tracing::{debug, error, info};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MotorController {
    id: u32,
    pins: [u8; 4],
    #[serde(skip)]
    last_a_value: Movement,
    #[serde(skip)]
    last_b_value: Movement,
    #[serde(skip)]
    tx_array: Vec<Sender<MotorControllerMessage>>,
    #[serde(skip)]
    rx_array: Vec<Receiver<MotorControllerMessage>>,
}

impl MotorController {
    pub fn create(id: u32, pins: [u8; 4]) -> Self {
        Self {
            id,
            pins,
            last_a_value: Movement::default(),
            last_b_value: Movement::default(),
            tx_array: vec![],
            rx_array: vec![],
        }
    }

    pub async fn set_motor<T: Read + Write>(
        &mut self,
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

    pub fn subscriber(&mut self) {}

    pub fn publisher(&mut self) {}
}

impl IOSensor for MotorController {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        debug!("registering motor controller: {:?}", self);
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
struct MotorControllerSubscriber {
    subscriber: Subscriber<MotorControllerMessage>,
}

impl MotorControllerSubscriber {
    pub async fn get(&mut self) -> Result<MotorControllerMessage> {
        self.subscriber.get().await
    }
}

pub struct MotorControllerPublisher {
    publisher: Publisher<MotorControllerMessage>,
}

impl MotorControllerPublisher {
    pub async fn set(&mut self, value: MotorControllerMessage) -> Result<()> {
        self.publisher.set(value).await
    }
}
