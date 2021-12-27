#[macro_use]
extern crate log;

use ascii::IntoAsciiString;
use color_eyre::eyre::Result;

//mod circuit;
mod loader;
mod sensor;
mod overseer;
mod serial;
mod Serial;
//use std::time::{Duration, Instant};



fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    let mut over_seer = overseer::OverSeer::new()?;
    over_seer.setup_system()?;

    //info!("starting up");
    //println!("Start");
    //let out = "{\"Ping\": true}".into_ascii_string()?;
    //println!("Connect");
    //let mut serial = serial::SerialComm::connect()?;
    //println!("Send");
    //serial.send(&out)?;
    //println!("Recieve");
    //let x = serial.recieve();
    //match x {
    //    Ok(y) => println!("FINAL: {}", y),
    //    Err(z) => println!("Error: {}", z),
    //}
    //println!("Done");
    Ok(())

    //let mut running = true;
    //let mut overseer = overseer::OverSeer::default();
    //overseer.init();
    //let mut last_update = Instant::now();
    //while running {
    //    let now = Instant::now();
    //    let delta_time = now - last_update;
    //    last_update = now;
    //    running = overseer.run(delta_time);
    //}
}
