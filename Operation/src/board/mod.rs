use getset::Getters;
use std::vec;
use tokio_serial::SerialPort;

pub mod arduino_board;
pub mod network;
pub mod serial_board;
use crate::sensor::IOSensor;
use crate::sensor::Sensor;
use color_eyre::eyre::{eyre, Result, WrapErr};
use tracing::{debug, error, info};

pub enum BoardTypes {
    SerialBoard(arduino_board::ArduinoBoard<Box<dyn SerialPort>>),
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct BoardWrapper {
    id: u32,
    board: BoardTypes,
    sensors: Vec<Sensor>,
}

impl BoardWrapper {
    pub fn new(id: u32, board: BoardTypes) -> Self {
        Self {
            id,
            board,
            sensors: vec![],
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }
}

pub struct BoardContainer {
    pub boards: Vec<BoardWrapper>,
}

impl BoardContainer {
    pub fn create(boards: Vec<BoardWrapper>) -> Self {
        Self { boards }
    }
}

impl BoardContainer {
    pub fn add_board(&mut self, board: BoardWrapper) {
        self.boards.push(board);
    }

    pub fn get_board_vec_size(&self) -> usize {
        self.boards.len()
    }

    pub async fn connect_sensors(&mut self) -> Result<()> {
        debug!(
            "Attempting to register boards, found {} boards to register",
            self.boards.len()
        );
        for board in self.boards.iter_mut() {
            debug!("Registering {}", board.id);
            let mut firmata_comm = match &board.board {
                BoardTypes::SerialBoard(v) => v.board.lock().await,
            };
            for sensor in board.sensors.iter_mut() {
                match sensor {
                    Sensor::Servo(v) => v.register(&mut firmata_comm)?,
                    Sensor::MotorController(v) => v.register(&mut firmata_comm)?,
                    Sensor::Motor(v) => v.register(&mut firmata_comm)?,
                    Sensor::PhotoResistor(v) => v.register(&mut firmata_comm)?,
                };
            }
        }
        Ok(())
    }
}
