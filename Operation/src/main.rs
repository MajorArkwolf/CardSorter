//#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
pub mod backbone;
pub mod board;
pub mod circuit;
pub mod factory;
pub mod io;
pub mod network;
pub mod sensor;
use futures::stream::FuturesUnordered;
use tokio::task::JoinHandle;
use tracing::{debug, info};
use tokio_stream::StreamExt;
use color_eyre::eyre::{eyre, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    version: String,
}

use crate::backbone::overseer::Overseer;
//use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = Args::parse();


    //console_subscriber::init();
    info!("System startup initiated");
    let mut overseer = Overseer::create();
    let overseer_channel = overseer.get_comm_channels();

    let mut system = factory::generate_system().await?;
    let mut circuit_watcher =
        circuit::factory::generate_circuits(overseer_channel, &mut system).await?;

    let mut main_tasks: FuturesUnordered<JoinHandle<Result<String>>> = FuturesUnordered::new();
    info!("Beginning circuit watcher task.");
    main_tasks.push(
        tokio::task::spawn(async move {
        circuit_watcher.run().await?;
        drop(circuit_watcher);
        Ok("circuit watcher tasked ended".to_string())
    }));

    info!("Beginning overseer task.");
    main_tasks.push(
        tokio::task::spawn(async move {
        overseer.run().await?;
        drop(overseer);
        Ok("overseer tasked ended".to_string())
    }));
    info!("Task creation complete, system running");

    while let Some(task) = main_tasks.next().await {
        match task {
            Ok(v) => {
                match v {
                    Ok(res) => {
                        debug!("Main Task Finished: {}", res);
                        
                    },
                    Err(err) => {
                        return Err(eyre!("task returned a failrue: {:?}", err));
                    },
                }
                
            },
            Err(err) => {
                return Err(eyre!("task failed to join, terminating program: {:?}", err));
            },
        }
    }
    info!("process and sub tasks succesfully ended, goodbye.");
    Ok(())
}
