pub mod motor;
pub mod motor_controller;
pub mod photo_resistor;
pub mod servo;
use color_eyre::eyre::Result;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone, EnumAsInner)]
pub enum Sensor {
    Servo(servo::Servo),
    MotorController(motor_controller::MotorController),
    Motor(motor::Motor),
    PhotoResistor(photo_resistor::PhotoResistor),
}

impl Sensor {
    pub fn get_id(&self) -> u32 {
        match self {
            Sensor::Servo(v) => v.get_id(),
            Sensor::MotorController(v) => v.get_id(),
            Sensor::Motor(v) => v.get_id(),
            Sensor::PhotoResistor(v) => v.get_id(),
        }
    }

    pub fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        match self {
            Sensor::Servo(v) => v.register(board),
            Sensor::MotorController(v) => v.register(board),
            Sensor::Motor(v) => v.register(board),
            Sensor::PhotoResistor(v) => v.register(board),
        }
    }
}

pub trait IOSensor {
    fn get_id(&self) -> u32;
    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()>;
}
