pub mod motor;
pub mod motor_controller;
pub mod photo_resistor;
pub mod servo;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use color_eyre::eyre::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Sensor {
    Servo(servo::Servo),
    MotorController(motor_controller::MotorController),
    Motor(motor::Motor),
    PhotoResistor(photo_resistor::PhotoResistor),
}

pub trait IOSensor {
    fn get_id(&self) -> u32;
    fn register<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()>;
}
