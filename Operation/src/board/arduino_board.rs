use std::clone::Clone;
use std::{
    io::{Read, Write},
    sync::Arc,
};

use color_eyre::eyre::{eyre, Result, WrapErr};
use firmata::Firmata;
use tokio::sync::Mutex;
pub type BoardHandle<T> = Arc<Mutex<firmata::Board<T>>>;

#[derive(Debug)]
pub struct ArduinoBoard<T: Read + Write> {
    pub board: BoardHandle<T>,
}

impl<T: Read + Write> Clone for ArduinoBoard<T> {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
        }
    }
}

impl<T: Read + Write> ArduinoBoard<T> {
    pub fn new(board: BoardHandle<T>) -> Self {
        Self { board }
    }

    pub async fn update(&mut self) -> Result<()> {
        match self
            .board
            .lock()
            .await
            .read()
            .wrap_err_with(|| "failed to update board")
        {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}
