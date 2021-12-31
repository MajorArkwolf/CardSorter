use tokio_serial::SerialPort;

use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct SerialTemplate {
    pub baud_rate: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EthernetTemplate {
    ip_address: String,
    port: u32,
}
