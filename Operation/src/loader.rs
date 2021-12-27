use std::fs::File;
use std::io::Read;
use serde_json::{json, from_str};
use color_eyre::eyre::{Result, WrapErr, eyre};

pub fn load_file(s: &str) -> Result<String> {
    let mut file = File::open(s).wrap_err_with(||eyre!("failed to open file: {}", s))?;
    let mut data = String::new();
    file.read_to_string(&mut data).wrap_err_with(||"failed to read file into string")?;
    Ok(data)
}