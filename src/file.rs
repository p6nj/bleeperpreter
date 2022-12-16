use std::fs;

use anyhow::{Context, Result};

pub fn read_file(filename: String) -> Result<String> {
    let data: String = fs::read_to_string(&filename)
        .with_context(|| format!("Error reading file {:?}", filename))?;
    let mut parsed_data: String = data
        .parse()
        .with_context(|| format!("Error parsing file {:?}", filename))?;
    parsed_data.pop(); // the last line is always empty
    Ok(parsed_data)
}
