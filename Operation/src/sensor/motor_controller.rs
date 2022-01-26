use color_eyre::eyre::{eyre, Result};
use firmata::asynchronous::board::Board;
use std::io::{Read, Write};
const MOTOR_A_PINS_INDEX: [usize; 2] = [0, 1];
const MOTOR_B_PINS_INDEX: [usize; 2] = [2, 3];
const MOTOR_FORWARD: [bool; 2] = [true, false];
const MOTOR_REVERSE: [bool; 2] = [true, false];
const MOTOR_STOP: [bool; 2] = [false, false];

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

#[derive(Clone, Debug)]
pub struct MotorController {
    id: u32,
    pins: [u8; 4],
    board: Board,
}

impl MotorController {
    pub fn create(id: u32, pins: [u8; 4], board: Board) -> Self {
        Self { id, pins, board }
    }

    pub async fn set_motor<T: Read + Write>(
        &mut self,
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
            self.board
                .digital_write(firmata::PinId::Digital(*pin), digital_assignment[i])
                .await?;
        }
        Ok(())
    }
}
