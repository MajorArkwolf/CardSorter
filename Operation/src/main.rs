mod firmata_comm;
use log::{debug, error, info};
use color_eyre::eyre::Result;
//use std::time::{Duration, Instant};



fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    Ok(())
}
