use color_eyre::eyre::{Result, WrapErr};
use firmata::asynchronous::board::Board;
use getset::Getters;
use std::fmt;

#[derive(Clone, Debug, Copy)]
pub struct PixelColor {
    pixel_positon: i32,
    red: u8,
    green: u8,
    blue: u8,
}

impl PixelColor {
    pub fn new(pixel_positon: i32, red: u8, green: u8, blue: u8) -> PixelColor {
        Self {
            pixel_positon,
            red,
            green,
            blue,
        }
    }
}

impl fmt::Display for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0},{1},{2},{3},",
            self.red, self.green, self.blue, self.pixel_positon
        )
    }
}

#[derive(Clone, Debug, Getters)]
pub struct LedStrip {
    #[get = "pub"]
    id: u32,
    board: Board,
}

impl LedStrip {
    pub fn create(id: u32, board: Board) -> Self {
        Self { id, board }
    }

    pub async fn set(&mut self, value: PixelColor) -> Result<()> {
        self.board
            .string_write(&value.to_string())
            .await
            .wrap_err_with(|| "failed to write led string to firmata board")
    }
}
