use color_eyre::eyre::{eyre, Result};

pub enum State {
    Offline,
    Ready,
    Scanning,
    Waiting,
}

pub trait Scanner {
    fn shutdown(&mut self) -> Result<()>;
    fn read(&mut self) -> Result<image::DynamicImage>;
    fn status(&mut self) -> Result<()>;
}
