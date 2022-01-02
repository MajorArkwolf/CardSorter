//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod board;
pub mod circuit;
pub mod factory;
pub mod sensor;
pub mod subscriber;
use color_eyre::eyre::{eyre, Context, ContextCompat, Error, Result};
use firmata::{Firmata, OutputMode, PinId};
use std::{process::Output, sync::Arc, thread};
use tokio::sync::Mutex;
use tracing::{debug, error, info};
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    //tracing_subscriber::fmt::init();
    console_subscriber::init();
    info!("System startup initiated");
    let board_array = Arc::new(Mutex::new(factory::generate_system()?));

    let mut sub = {
        let mut mutex = board_array.lock().await;
        assert!(mutex.get_board_vec_size() != 0);
        mutex.connect_sensors().await?;

        match mutex.get_sensor(2)? {
            sensor::Sensor::Servo(_) => todo!(),
            sensor::Sensor::MotorController(_) => todo!(),
            sensor::Sensor::Motor(_) => todo!(),
            sensor::Sensor::PhotoResistor(v) => v.subscribe(),
        }
    };

    let mut publ = {
        let mut mutex = board_array.lock().await;
        match mutex.get_sensor(1)? {
            sensor::Sensor::Servo(v) => v.publisher(),
            sensor::Sensor::MotorController(_) => todo!(),
            sensor::Sensor::Motor(_) => todo!(),
            sensor::Sensor::PhotoResistor(_) => todo!(),
        }
    };

    info!("System startup complete, beginning run process");
    let join = tokio::task::spawn(async move {
        loop {
            let mut data = board_array.lock().await;
            data.update().await?;
        }
        Ok::<(), Error>(())
    });

    let mut i = 0;

    let mut dur = std::time::Instant::now();

    loop {
        //debug!("Beginning data update.");
        //debug!("Attempting to find message.");
        let x = sub.get().await?;

        if dur.elapsed() > std::time::Duration::from_secs(3) {
            i += 90;
            if i > 180 {
                i = 0;
            }
            debug!("hello :3 uwu");
            publ.set(i).await?;
            dur = std::time::Instant::now();
        }
    }
    join.await??;
    info!("System shutdown successfully");
    Ok(())
}
