use std::io::{Read, Write};

use color_eyre::eyre::{Result, WrapErr, eyre};


pub struct FirmataComm<T: Read + Write + Sized> {
    pub device: firmata::Board<T>,
}

impl<T: Read + Write + Sized> FirmataComm<T> {
    pub fn new(connection: T) -> Result<Self> {
        let device = firmata::Board::new(Box::new(connection)).wrap_err_with(||"failed to create firmata device comms")?;
        Ok(FirmataComm{device})
    }
}