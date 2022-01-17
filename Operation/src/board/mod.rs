use firmata::Firmata;
use getset::Getters;
use std::collections::HashMap;
use std::vec;
use tokio_serial::SerialPort;
pub mod arduino_board;
pub mod network;
pub mod serial_board;
use crate::sensor::Sensor;
use color_eyre::eyre::{eyre, Result};
use tracing::debug;

pub enum BoardTypes {
    SerialBoard(arduino_board::ArduinoBoard<Box<dyn SerialPort>>),
}

impl core::fmt::Debug for BoardTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerialBoard(_arg0) => f.debug_tuple("SerialBoard").finish(),
        }
    }
}

#[derive(Getters, Debug)]
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

    pub fn get_board(&mut self) -> &mut BoardTypes {
        &mut self.board
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    pub fn get_sensor(&mut self, id: u32) -> Result<&mut Sensor> {
        match self.sensors.iter_mut().find(|x| x.get_id() == id) {
            Some(v) => Ok(v),
            None => Err(eyre!(
                "failed to find sensor id `{}` inside board `{}`",
                id,
                self.id
            )),
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        let mut temp_board = match &self.board {
            BoardTypes::SerialBoard(v) => v.board.lock().await,
        };
        match temp_board.poll(2) {
            Ok(_) => {}
            Err(e) => match e {
                firmata::FirmataError::Timeout(_) => {}
                firmata::FirmataError::ParseError(_, _) => {}
                firmata::FirmataError::IoError(_) => {}
                err => return Err(eyre!("firmata board errer: {:?}", err)),
            },
        };
        for sensor in self.sensors.iter_mut() {
            match sensor {
                Sensor::Servo(v) => v.update(&mut *temp_board).await?,
                Sensor::MotorController(v) => v.update(&mut *temp_board).await?,
                Sensor::LedStrip(v) => v.update(&mut *temp_board).await?,
                Sensor::PhotoResistor(v) => v.update(&mut *temp_board).await?,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct BoardContainer {
    pub boards: Vec<BoardWrapper>,
    id_map: HashMap<u32, u32>,
}

impl BoardContainer {
    pub fn create(boards: Vec<BoardWrapper>) -> Self {
        let mut id_map: HashMap<u32, u32> = HashMap::new();
        for board in boards.iter() {
            for sensor in board.sensors() {
                id_map.insert(sensor.get_id(), board.id);
            }
        }
        Self { boards, id_map }
    }

    pub async fn update(&mut self) -> Result<()> {
        for board in self.boards.iter_mut() {
            board.update().await?
        }
        Ok(())
    }

    pub fn add_board(&mut self, board: BoardWrapper) {
        self.boards.push(board);
    }

    pub fn get_board_vec_size(&self) -> usize {
        self.boards.len()
    }

    pub fn get_sensor(&mut self, id: u32) -> Result<&mut Sensor> {
        let board_id = match self.id_map.get(&id) {
            Some(v) => *v,
            None => return Err(eyre!("sensor id `{}` was not found in the system", id)),
        };

        self.boards
            .iter_mut()
            .find(|x| x.id == board_id)
            .ok_or_else(|| eyre!("failed to find"))?
            .get_sensor(id)
    }

    pub fn get_board_from_sensor_id(&mut self, id: u32) -> Result<&mut BoardWrapper> {
        let board_id = match self.id_map.get(&id) {
            Some(v) => *v,
            None => return Err(eyre!("sensor id `{}` was not found in the system", id)),
        };

        self.boards
            .iter_mut()
            .find(|x| x.id == board_id)
            .ok_or_else(|| eyre!("failed to find"))
    }

    pub async fn connect_sensors(&mut self) -> Result<()> {
        debug!(
            "Attempting to register boards, found {} boards to register.",
            self.boards.len()
        );
        for board in self.boards.iter_mut() {
            debug!("Registering {}.", board.id);
            let mut firmata_comm = match &board.board {
                BoardTypes::SerialBoard(v) => v.board.lock().await,
            };
            for sensor in board.sensors.iter_mut() {
                sensor.register(&mut firmata_comm)?;
            }
        }
        debug!("Sensors registered succesfully.");
        Ok(())
    }
}
