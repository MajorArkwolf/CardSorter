#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
mod firmata_comm;
mod job_queue;
mod sensor;

use std::{convert::TryInto, sync::Arc};

use crate::firmata_comm::FirmataComm;
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use tracing::{debug, error, info};
//use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let ports =
        tokio_serial::available_ports().wrap_err_with(|| "failed to find any comm ports")?;

    let port = ports
        .into_iter()
        .map(|v| tokio_serial::new(&v.port_name, 9600).open())
        .last()
        .ok_or_else(|| eyre!("failed to find comm port"))??;

    let cake = job_queue::SerialJobQueue::new(port);
    let x = cake.make_async_serial();
    let comm = FirmataComm::connect(x);

    //let valid_port = for port in ports {
    //    let serial = tokio_serial::new(port.port_name, 9600).open();
    //    match serial {
    //        Ok(v) => v,
    //        Err(_) => continue,
    //    }
    //
    //};

    Ok(())
}
