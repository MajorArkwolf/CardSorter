pub mod motor;
pub mod motor_controller;
pub mod servo;
use std::io::{Read, Write};

use async_trait::async_trait;
use color_eyre::eyre::Result;

#[async_trait]
pub trait DigitalIo<T: Read + Write + ?Sized> {
    async fn get(&mut self) -> Result<bool>;
    async fn set(&mut self, value: bool) -> Result<()>;
}

#[async_trait]
pub trait AnalogIo<T: Read + Write + ?Sized> {
    async fn get(&mut self) -> Result<i32>;
    async fn set(&mut self, value: i32) -> Result<()>;
}
