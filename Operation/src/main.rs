//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod circuit;
pub mod factory;
pub mod network;
pub mod sensor;
use color_eyre::eyre::Result;
use tracing::{debug, info};

use crate::circuit::circuit_constructor;
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    //console_subscriber::init();
    info!("System startup initiated");

    let mut system = factory::generate_system().await?;
    let calibration_results = circuit_constructor::calibrate_sensors(&mut system).await?;
    let mut circuit_controller =
        circuit_constructor::construct_circuit(&mut system, &calibration_results)?;

    circuit_controller.start().await?;
    debug!("Beginning circuit controller update cycle");
    loop {
        circuit_controller.update().await?;
    }
    Ok(())
}
