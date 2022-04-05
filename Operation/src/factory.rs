use crate::board::{generate_board_io, BoardTemplate, FirmataBoardTask};
use color_eyre::eyre::{eyre, Context, Result};
use std::io::Read;
use std::{
    fs::{File},
};

use tracing::info;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
pub struct System {
    pub board_tasks: Vec<FirmataBoardTask>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemTemplate {
    boards: Vec<BoardTemplate>,
}

async fn generate_board(template: BoardTemplate) -> Result<FirmataBoardTask> {
    let board = generate_board_io(template.address).await?;
    Ok(FirmataBoardTask::create(template.id, board))
}

pub async fn generate_system() -> Result<System> {
    info!("Beginning system generation...");
    // Load our structure in from json to begin construction
    let mut file = File::open("./system.json")
        .wrap_err_with(|| "failed to find system.json file in the root directory")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let template: SystemTemplate = serde_json::from_str(&contents)?;

    let mut system = System::default();

    // Generate the boards we will be interfacing with
    info!("Generating boards...");
    for temp_board in template.boards {
        if system.board_tasks.iter().any(|x| *x.id() == temp_board.id) {
            return Err(eyre!("board id already exists, {:?}", temp_board));
        }
        system.board_tasks.push(generate_board(temp_board).await?);
    }
    info!("Generating boards complete.");
    info!(
        "Found {} boards that were setup succesfully.",
        system.board_tasks.len()
    );
    Ok(system)
}
