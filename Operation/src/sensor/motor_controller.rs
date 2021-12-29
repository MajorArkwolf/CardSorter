use crate::{arduino_board::ArduinoBoard, sensor::AnalogIo};
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task};

const MOTOR_A_PINS_INDEX: [usize; 2] = [0, 1];
const MOTOR_B_PINS_INDEX: [usize; 2] = [2, 3];
const MOTOR_FORWARD: [i32; 2] = [1, 0];
const MOTOR_REVERSE: [i32; 2] = [0, 1];
const MOTOR_STOP: [i32; 2] = [0, 0];

pub enum Motor {
    A,
    B,
}

pub enum Movement {
    Stop,
    Forward,
    Reverse,
}

#[derive(Clone, Debug)]
pub struct MotorController<T: Read + Write + ?Sized> {
    board: ArduinoBoard<T>,
    pins: [i32; 4],
}

impl<T: Read + Write + ?Sized> MotorController<T> {
    pub async fn create(board: ArduinoBoard<T>, pins: [i32; 4]) -> Result<Self> {
        for pin in pins {
            board
                .board
                .lock()
                .await
                .set_pin_mode(pin, firmata::OUTPUT)
                .wrap_err_with(|| "failed to create motor controller")?;
        }
        Ok(Self { board, pins })
    }

    pub async fn set_motor(&mut self, motor: Motor, movement: Movement) -> Result<()> {
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
        let mut x = self.board.board.lock().await;
        for (i, pin) in pins.iter().enumerate() {
            x.digital_write(*pin, digital_assignment[i]);
        }
        Ok(())
    }
}
