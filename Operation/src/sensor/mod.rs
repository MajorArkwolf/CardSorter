use self::{motor_controller::MotorController, photo_resistor::PhotoResistor, servo::Servo};
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
pub mod led_strip;
//pub mod motor;
pub mod motor_controller;
pub mod photo_resistor;
pub mod servo;

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum Type {
    MotorController,
    PhotoResistor,
    Servo,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SensorTemplate {
    pub id: u32,
    pub board_id: u32,
    pub sensor_type: Type,
    pub pins: Vec<u8>,
}

#[derive(Getters, Setters, MutGetters, Debug, Clone, Default)]
pub struct SensorContainer {
    pub motor_controllers: Vec<MotorController>,
    pub photo_resistor: Vec<PhotoResistor>,
    pub servos: Vec<Servo>,
}

impl SensorContainer {
    pub fn find_sensor_by_id(&self, id: u32) -> Type {
        let found = self.motor_controllers.iter().any(|y| *y.id() == id);
        if found {
            return Type::MotorController;
        }

        let found = self.photo_resistor.iter().any(|y| *y.id() == id);
        if found {
            return Type::PhotoResistor;
        }

        let found = self.servos.iter().any(|y| *y.id() == id);
        if found {
            return Type::Servo;
        }

        return Type::None;
    }
}
