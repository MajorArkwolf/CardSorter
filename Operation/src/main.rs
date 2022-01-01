//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod factory;
pub mod sensor;
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use firmata::Firmata;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("beginning startup of system");
    let board_array = Arc::new(Mutex::new(factory::generate_system()?));
    {
        let mut mutex = board_array.lock().await;
        assert!(mutex.get_board_vec_size() != 0);
        mutex.connect_sensors().await?;
    }

    info!("terminating");
    Ok(())
}
