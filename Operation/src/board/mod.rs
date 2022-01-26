use super::network::connect;
use color_eyre::eyre::{eyre, Context, Result};

use firmata::asynchronous::boardio::BoardIo;
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    ToSocketAddrs,
};

pub async fn generate_board_io<A: ToSocketAddrs>(
    addr: A,
) -> Result<BoardIo<OwnedReadHalf, OwnedWriteHalf>> {
    let stream = connect(addr).await?;
    let (r, w) = stream.into_split();
    let mut board = BoardIo::create(r, w);
    board
        .generate_board_state()
        .await
        .wrap_err_with(|| eyre!("failed to generate board state when constructing BoardIo"))?;
    Ok(board)
}
