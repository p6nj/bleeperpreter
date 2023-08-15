use anyhow::{Context, Error};
use json::JsonValue;
use std::{fs::read_to_string, path::Path};
pub(crate) fn parse<P: AsRef<Path>>(file: P) -> Result<JsonValue, Error> {
    validate(json::parse(
        read_to_string(file)
            .context("error reading json file")?
            .as_str(),
    )?)
}

fn validate(json: JsonValue) -> Result<JsonValue, Error> {
    Ok(json)
}
