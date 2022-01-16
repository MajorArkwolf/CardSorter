use super::IOSensor;
use crate::subscriber;
use async_channel::Receiver;
use color_eyre::eyre::{Result, WrapErr};
use firmata::Firmata;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::format;
use std::io::{Read, Write};
use subscriber::Publisher;
use tracing::error;
use tracing::instrument;

#[derive(Copy, Clone, Debug)]
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
            "{}",
            format!(
                "{}{} {} {} {} {}",
                '{', self.red, self.green, self.blue, self.pixel_positon, '}'
            )
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LedStrip {
    id: u32,
    #[serde(skip)]
    rx_array: Vec<Receiver<PixelColor>>,
}

impl LedStrip {
    pub fn create(id: u32) -> Self {
        Self {
            id,
            rx_array: vec![],
        }
    }

    pub fn publisher(&mut self) -> LedPublisher {
        let (tx, rx) = async_channel::bounded::<PixelColor>(20);
        self.rx_array.push(rx);

        LedPublisher {
            publisher: Publisher::create(tx),
        }
    }

    pub fn set<T: Read + Write>(
        &mut self,
        board: &mut firmata::Board<T>,
        value: PixelColor,
    ) -> Result<()> {
        board
            .string_write(&value.to_string())
            .wrap_err_with(|| "failed to write led string to firmata board")
    }

    pub async fn update<T: Read + Write>(&mut self, board: &mut firmata::Board<T>) -> Result<()> {
        for channel in self.rx_array.iter() {
            if !channel.is_empty() && !channel.is_closed() {
                let value = match channel.recv().await {
                    Ok(v) => v,
                    Err(_) => {
                        error!("recv error when listening to publishers");
                        continue;
                    }
                };
                board
                    .string_write(&value.to_string())
                    .wrap_err_with(|| "failed to write led string to firmata board")?;
            }
        }
        Ok(())
    }
}

impl IOSensor for LedStrip {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn register<T: Read + Write>(&mut self, _board: &mut firmata::Board<T>) -> Result<()> {
        // This is set on the board as its a non standard setup
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct LedPublisher {
    publisher: Publisher<PixelColor>,
}

impl LedPublisher {
    #[instrument]
    pub async fn set(&mut self, value: PixelColor) -> Result<()> {
        self.publisher.set(value).await
    }
}
