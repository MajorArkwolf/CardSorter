use super::network::connect;
use color_eyre::eyre::{eyre, Context, Result};
use firmata::asynchronous::{board::Board, boardio::BoardIo};
use getset::Getters;
use serde::{Deserialize, Serialize};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        ToSocketAddrs,
    },
    task::JoinHandle,
};

pub type FirmataBoardIo = BoardIo<OwnedReadHalf, OwnedWriteHalf>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardTemplate {
    pub id: u32,
    pub identifier: String,
    pub address: String,
}

#[derive(Getters, Debug)]
pub struct FirmataBoardTask {
    #[get = "pub"]
    id: u32,
    #[get = "pub"]
    board: Board,
    #[get = "pub"]
    task: JoinHandle<()>,
}

impl FirmataBoardTask {
    pub fn create(id: u32, board_io: FirmataBoardIo) -> Self {
        let board = board_io.get_board();
        let task = tokio::task::spawn(async move {
            board_io.poll().await;
        });
        Self { id, board, task }
    }
}

pub async fn generate_board_io<A: ToSocketAddrs>(addr: A) -> Result<FirmataBoardIo> {
    let stream = connect(addr).await?;
    let (r, w) = stream.into_split();
    let mut board = BoardIo::create(r, w);
    board
        .generate_board_state()
        .await
        .wrap_err_with(|| eyre!("failed to generate board state when constructing BoardIo"))?;
    Ok(board)
}
