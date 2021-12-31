use getset::Getters;
use std::vec;
use tokio_serial::SerialPort;

pub mod arduino_board;
pub mod network;
pub mod serial_board;
use crate::sensor::Sensor;

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
}

pub struct BoardContainer {
    pub boards: Vec<BoardWrapper>,
}

impl Default for BoardContainer {
    fn default() -> Self {
        Self {
            boards: Default::default(),
        }
    }
}

impl BoardContainer {
    pub fn add_board(&mut self, board: BoardWrapper) {
        self.boards.push(board);
    }

    pub fn get_board_vec_size(&self) -> usize {
        self.boards.len()
    }
}
