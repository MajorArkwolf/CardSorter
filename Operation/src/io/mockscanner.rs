use std::fs::{self, DirEntry};

use super::scanner::Scanner;
use color_eyre::eyre::{eyre, Result};

pub enum Mode {
    Select,
    InOrder,
    Random,
}

fn get_all_picture_files(folder_path: &std::path::Path) -> Result<()> {
    let paths = fs::read_dir(folder_path)?;
    let files: Vec<DirEntry> = paths.filter_map(|f| f.ok()).collect();
    Ok(())
}

struct MockScanner {
    files: Vec<std::path::PathBuf>,
}

impl MockScanner {
    fn init(mode: Mode, folder_path: &std::path::Path) -> Result<Self> {
        match mode {
            Mode::Select => Ok(Self { files: Vec::new() }),
            Mode::InOrder => todo!(),
            Mode::Random => todo!(),
        }
    }

    fn add_to_front(&mut self, file_path: &std::path::Path) {
        let old_files = self.files.clone();
        self.files = vec![file_path.to_path_buf()];
        self.files.extend(old_files);
    }
}

impl Scanner for MockScanner {
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
