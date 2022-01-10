use crate::board::network::SerialTemplate;

use super::{arduino_board::ArduinoBoard, BoardTypes};
use color_eyre::eyre::{eyre, Result, WrapErr};
use core::time;
use firmata::Firmata;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_serial::{DataBits, Parity, SerialPort, StopBits};
use tracing::error;

pub fn generate(port: Box<dyn SerialPort>) -> ArduinoBoard<Box<dyn SerialPort>> {
    ArduinoBoard::new(Arc::new(Mutex::new(firmata::Board::new(port))))
}

pub fn generate_serial_board(template: SerialTemplate, identifier: String) -> Result<BoardTypes> {
    let mut devices_found: String = String::new();
    let ports =
        tokio_serial::available_ports().wrap_err_with(|| "failed to find any comm ports")?;
    let mut tried = 0;
    let mut skipped = 0;
    for p in ports {
        let comm = match tokio_serial::new(&p.port_name, template.baud_rate)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .open()
        {
            Ok(x) => x,
            Err(_) => {
                skipped += 1;
                devices_found.push(' ');
                devices_found.push_str(&p.port_name);
                continue;
            }
        };
        tried += 1;
        comm.clear(tokio_serial::ClearBuffer::All)?;
        let mut temp_board = firmata::Board::new(comm);
        temp_board.populate_board_info()?;
        temp_board.sampling_inerval(std::time::Duration::from_millis(100))?;
        let board_name = temp_board.firmware_name();
        if board_name == identifier {
            let board = ArduinoBoard::new(Arc::new(Mutex::new(temp_board)));
            return Ok(BoardTypes::SerialBoard(board));
        } else {
            devices_found.push(' ');
            devices_found.push_str(board_name);
        }
    }

    error!(
        "Tried: {}, Skipped {}. Failed to find `{}`: found `{}`",
        tried, skipped, identifier, devices_found
    );
    Err(eyre!("failed to find board"))
}
