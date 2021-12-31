use crate::board::{network::SerialTemplate, BoardWrapper};

use super::{arduino_board::ArduinoBoard, BoardTypes};
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::sync::Mutex;
use tokio_serial::SerialPort;
use tracing::{debug, error, info};

pub fn generate(port: Box<dyn SerialPort>) -> ArduinoBoard<Box<dyn SerialPort>> {
    ArduinoBoard::new(Arc::new(Mutex::new(firmata::Board::new(port))))
}

pub fn generate_serial_board(template: SerialTemplate, identifier: String) -> Result<BoardTypes> {
    let mut devices_found: String = String::new();
    let ports =
        tokio_serial::available_ports().wrap_err_with(|| "failed to find any comm ports")?;
    for p in ports {
        let p = match tokio_serial::new(p.port_name, template.baud_rate).open() {
            Ok(x) => x,
            Err(_) => continue,
        };
        let mut temp_board = firmata::Board::new(p);
        temp_board.populate_board_info()?;
        let board_name = temp_board.firmware_name();
        if board_name == identifier {
            let board = ArduinoBoard::new(Arc::new(Mutex::new(temp_board)));
            return Ok(BoardTypes::SerialBoard(board));
        } else {
            devices_found.push(' ');
            devices_found.push_str(board_name);
        }
    }

    error!("Failed to find `{}`: found `{}`", identifier, devices_found);
    Err(eyre!("failed to find board"))
}
