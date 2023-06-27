use anyhow::{Context, Error};
use json::JsonValue;
use std::fs::read_to_string;
pub fn parse(filename: String) -> Result<JsonValue, Error> {
    Ok(validate(JsonValue::from(
        read_to_string(filename).context("error reading json file")?,
    ))?)
}

fn validate(json: JsonValue) -> Result<JsonValue, Error> {
    Ok(json)
}
