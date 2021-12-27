use std::io::{Read, Write};

use color_eyre::eyre::{eyre, Result, WrapErr};

pub struct FirmataComm<T: Read + Write> {
    pub device: firmata::Board<T>,
}

impl<T: Read + Write> FirmataComm<T> {
    pub fn connect(connection: T) -> Result<Self> {
        let device = firmata::Board::new(Box::new(connection))
            .wrap_err_with(|| "failed to create firmata device comms")?;
        Ok(Self { device })
    }
}
