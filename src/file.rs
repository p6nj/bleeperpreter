use std::fs;

use anyhow::{Context, Result};

pub fn read_file(filename: String) -> Result<String> {
    let data: String = fs::read_to_string(filename).context("error reading the file data")?;
    let mut parsed_data: String = data.parse().context("error parsing the file data")?;
    parsed_data.pop(); // the last line is always empty
    Ok(parsed_data)
}
