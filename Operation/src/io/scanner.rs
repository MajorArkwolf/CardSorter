use color_eyre::eyre::{eyre, Result};

pub enum State {
    Offline,
    Ready,
    Scanning,
    Waiting,
}

pub trait Scanner {
    fn setup(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    fn read(&mut self);
    fn status(&mut self) -> Result<()>;
}
