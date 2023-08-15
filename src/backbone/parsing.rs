use self::logos::Extras;
pub(crate) use self::logos::MaskAtom;
use super::{Album, Channel, Instrument, Mask, Root, Track};
use ::logos::Logos;
use anyhow::{Context, Error, Result};
use json::JsonValue;
use meval::Expr;
use std::{collections::HashMap, str::FromStr};
use text_lines::TextLines as TextPosition;
mod logos;
mod parsing_errors;
#[cfg(test)]
mod tests;

fn err_field(field: &str, r#type: &str) -> String {
    format!("field \"{}\" is not a \"{}\"", field, r#type)
}

impl TryFrom<&JsonValue> for Instrument {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value.entries().next().context("empty instrument")?.0 {
            "expr" => Ok(Instrument::Expression {
                expr: Expr::from_str(
                    value["expr"]
                        .as_str()
                        .context(err_field("expr", "string"))?,
                )
                .context("invalid instrument expression")?,
            }),
            _ => Err(Error::msg("unknown instrument type")),
        }
    }
}

impl TryFrom<&JsonValue> for Channel {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Error> {
        Ok(Channel {
            instrument: (&value["instrument"]).try_into()?,
            tuning: value["tuning"]
                .as_f32()
                .context(err_field("tuning", "unsigned 16-bit integer"))?,
            mask: Mask::try_from(value)?,
        })
    }
}

/// From the string mask and a string of allowed notes
impl TryFrom<&JsonValue> for Mask {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let notes = value["notes"]
            .as_str()
            .context("the \"notes\" field is not a string")?;
        Ok(Mask(
            notes.len().try_into()?,
            MaskAtom::lexer_with_extras(
                value["mask"]
                    .as_str()
                    .context(err_field("mask", "string"))?,
                Extras::new(
                    notes.to_string(),
                    TextPosition::new(value["mask"].as_str().unwrap()),
                ),
            )
            .inspect(|e| {
                if let Err(e) = e {
                    eprintln!("Warning: {:?}", e);
                }
            })
            .flatten()
            .collect(),
        ))
    }
}

impl TryFrom<&JsonValue> for Track {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        Ok(Track::new(
            value["BPM"].as_u16().context(err_field("BPM", "u16"))?,
            value["channels"]
                .entries()
                .map(move |(name, chan)| -> Result<(String, Channel)> {
                    Ok((name.to_string(), chan.try_into()?))
                })
                .collect::<Result<HashMap<String, Channel>>>()?,
        ))
    }
}

impl TryFrom<&JsonValue> for Album {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        Ok(Album::new(
            value["artist"]
                .as_str()
                .context(err_field("artist", "string"))?
                .to_string(),
            value["tracks"]
                .entries()
                .map(move |(name, track)| -> Result<(String, Track)> {
                    Ok((name.to_string(), track.try_into()?))
                })
                .collect::<Result<HashMap<String, Track>>>()?,
        ))
    }
}

impl TryFrom<JsonValue> for Root {
    type Error = Error;
    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Root(
            value
                .entries()
                .map(move |(name, album)| -> Result<(String, Album)> {
                    Ok((name.to_string(), album.try_into()?))
                })
                .collect::<Result<HashMap<String, Album>>>()?,
        ))
    }
}
