use super::scanner::Scanner;
use color_eyre::eyre::{eyre, Result};

struct NeatScanner {}

impl NeatScanner {
    fn setup(&mut self) -> color_eyre::Result<()> {
        todo!()
    }
}

impl Scanner for NeatScanner {
    fn shutdown(&mut self) -> color_eyre::Result<()> {
        todo!()
    }

    fn read(&mut self) -> Result<image::DynamicImage> {
        todo!()
    }

    fn status(&mut self) -> color_eyre::Result<()> {
        todo!()
    }
}
