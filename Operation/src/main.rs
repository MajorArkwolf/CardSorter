//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod circuit;
pub mod factory;
pub mod network;
pub mod sensor;
pub mod subscriber;
use circuit::circuit_controller::CircuitController;
use color_eyre::eyre::{Error, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::circuit::circuit_constructor;
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    //tracing_subscriber::fmt::init();
    console_subscriber::init();
    info!("System startup initiated");
    let board_array = Arc::new(Mutex::new(factory::generate_system()?));

    {
        let mut mutex = board_array.lock().await;
        assert!(mutex.get_board_vec_size() != 0);
        mutex.connect_sensors().await?;
    }

    let mut circuit_controller: CircuitController = {
        let mut mutex = board_array.lock().await;
        circuit_constructor::construct_circuit(&mut mutex)?
    };

    info!("System startup complete, beginning run process");
    let join = tokio::task::spawn(async move {
        loop {
            let mut data = board_array.lock().await;
            data.update().await?;
        }
        Ok::<(), Error>(())
    });

    circuit_controller.start().await?;
    debug!("Beginning circuit controller update cycle");
    loop {
        circuit_controller.update().await?;
    }
    Ok(())
}
