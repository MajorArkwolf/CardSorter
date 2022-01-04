//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod circuit;
pub mod factory;
pub mod sensor;
pub mod subscriber;
use circuit::circuit_constructor::construct_capture;
use circuit::circuit_controller::CircuitController;
use color_eyre::eyre::{eyre, Context, ContextCompat, Error, Result};
use firmata::{Firmata, OutputMode, PinId};
use std::{process::Output, sync::Arc, thread};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

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

    let circuit_controller: CircuitController = {
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

    let join = tokio::task::spawn(async move {
        circuit_controller.start().await?;
        loop {
            circuit_controller.update().await?;
        }
        Ok::<(), Error>(())
    });

    //join.await??;
    info!("System shutdown successfully");
    Ok(())
}
