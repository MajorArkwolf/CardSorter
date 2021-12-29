use std::{
    io::{Read, Write},
    sync::Arc,
};

use color_eyre::eyre::{eyre, Result, WrapErr};
use tokio::sync::Mutex;
pub type BoardHandle<T> = Arc<Mutex<firmata::Board<T>>>;
#[derive(Debug)]
pub struct ArduinoBoard<T: Read + Write + ?Sized> {
    pub board: BoardHandle<T>,
}

impl<T: Read + Write + ?Sized> Clone for ArduinoBoard<T> {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
        }
    }
}

impl<T: Read + Write + ?Sized> ArduinoBoard<T> {
    pub fn new(board: BoardHandle<T>) -> Self {
        Self { board }
    }
}
