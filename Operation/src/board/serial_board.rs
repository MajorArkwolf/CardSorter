use crate::board::network::SerialTemplate;

use super::{arduino_board::ArduinoBoard, BoardTypes};
use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_serial::{DataBits, Parity, SerialPort, SerialPortInfo, StopBits};
use tracing::{error, warn};

pub fn generate(port: Box<dyn SerialPort>) -> ArduinoBoard<Box<dyn SerialPort>> {
    ArduinoBoard::new(Arc::new(Mutex::new(firmata::Board::new(port))))
}

fn convert_dir_to_path(path: &str) -> Result<String> {
    let tokens = path.split('/').collect::<Vec<&str>>();
    let mut linux_path: String = "/dev/".to_owned();
    let name = tokens
        .last()
        .ok_or_else(|| eyre!("no last element found"))?;
    linux_path.push_str(name);
    Ok(linux_path)
}

fn generate_port(port_info: &SerialPortInfo, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    match tokio_serial::new(&port_info.port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open()
    {
        Ok(x) => Ok(x),
        Err(_) => match convert_dir_to_path(&port_info.port_name) {
            Ok(n) => match tokio_serial::new(&port_info.port_name, baud_rate)
                .data_bits(DataBits::Eight)
                .parity(Parity::None)
                .stop_bits(StopBits::One)
                .open()
            {
                Ok(v) => Ok(v),
                Err(e) => Err(eyre!(
                    "failed to connect to port after changing path path name: {} error: {:?}",
                    n,
                    e
                )),
            },
            Err(_) => Err(eyre!("failed to generate a new path name")),
        },
    }
}

pub fn generate_serial_board(template: SerialTemplate, identifier: String) -> Result<BoardTypes> {
    let mut devices_found: String = String::new();
    let ports =
        tokio_serial::available_ports().wrap_err_with(|| "failed to find any comm ports")?;
    let mut tried = 0;
    let mut skipped = 0;
    for p in ports {
        let comm = match generate_port(&p, template.baud_rate) {
            Ok(v) => v,
            Err(e) => {
                warn!("Comm port failed: {:?}", e);
                skipped += 1;
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
        "Tried: {}, Skipped {}. Failed to find `{}`",
        tried, skipped, identifier
    );
    Err(eyre!("failed to find board"))
}
