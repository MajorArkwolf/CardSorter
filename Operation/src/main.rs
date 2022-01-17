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
use tokio::sync::watch;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::circuit::circuit_constructor;
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    //console_subscriber::init();
    info!("System startup initiated");
    let board_array = Arc::new(Mutex::new(factory::generate_system()?));

    {
        let mut mutex = board_array.lock().await;
        assert!(mutex.get_board_vec_size() != 0);
        mutex.connect_sensors().await?;
    }

    let mut circuit_controller: CircuitController = {
        let mut mutex = board_array.lock().await;
        let calibration_results = circuit_constructor::calibrate_sensors(&mut mutex).await?;
        circuit_constructor::construct_circuit(&mut mutex, &calibration_results)?
    };

    let (tx, mut rx) = watch::channel(true);

    info!("System startup complete, beginning run process");
    let join = tokio::task::spawn(async move {
        loop {
            let mut data = board_array.lock().await;
            match data.update().await {
                Ok(_) => tx.send(true)?,
                Err(e) => {
                    tx.send(false)?;
                    return Err(e);
                }
            }
        }
        Ok::<(), Error>(())
    });

    circuit_controller.start().await?;
    debug!("Beginning circuit controller update cycle");
    loop {
        circuit_controller.update().await?;
        rx.changed().await?;
        if !*rx.borrow() {
            break;
        }
    }
    join.await??;
    Ok(())
}
