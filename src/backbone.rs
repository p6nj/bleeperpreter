use derive_new::new;
use hound::WavReader;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{Context, Error, Ok};
use json::JsonValue;
use meval::Expr;
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::multi::many0;
use nom::{character::complete::one_of, combinator::map_res, sequence::preceded, IResult};

pub type Samples = Vec<i16>;

pub struct Root(HashMap<String, Album>);

#[derive(new)]
pub struct Album {
    pub artist: String,
    pub tracks: HashMap<String, Track>,
}

#[derive(new)]
pub struct Track {
    pub bpm: u16,
    pub channels: HashMap<String, Channel>,
}

#[derive(PartialEq, Clone)]
pub enum Instrument {
    Sample {
        data: Samples,
        r#loops: bool,
        resets: bool,
    },
    Expression {
        expr: Expr,
        resets: bool,
    },
}

impl Debug for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instrument::Sample {
                data,
                r#loops,
                resets,
            } => write!(
                f,
                "Sample {{ data: ({:?}), loops: {:?}, resets: {:?} }}",
                data.len(),
                r#loops,
                resets
            ),
            Instrument::Expression { expr, resets } => {
                write!(f, "Expression {{ expr: {:?}, resets: {:?} }}", expr, resets)
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Channel {
    pub instrument: Instrument,
    pub tuning: u16,
    pub mask: Mask,
}

#[derive(PartialEq, Debug)]
enum MaskAtoms {
    Octave(u8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

#[derive(Debug, PartialEq)]
pub struct Mask(Vec<MaskAtoms>);

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(one_of(notes), move |c| {
        Ok::<MaskAtoms>(MaskAtoms::Note(
            notes
                .find(c)
                .with_context(|| format!("unknown note: {}", c))?
                .try_into()
                .context("converting note index to u8 (shouldn't fail)")?,
        ))
    })
}

fn rest<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(char('.'), move |_| Ok::<MaskAtoms>(MaskAtoms::Rest))
}

fn length<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('$'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms>(MaskAtoms::Length(n))
    })
}

fn octave<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('@'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms>(MaskAtoms::Octave(n))
    })
}

fn volume<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('!'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms>(MaskAtoms::Volume(n))
    })
}

fn err_field(field: &str, r#type: &str) -> String {
    format!("field \"{}\" is not a \"{}\"", field, r#type)
}

fn err_parse(r#type: &str, name: &str) -> String {
    format!("can't parse {} \"{}\"", r#type, name)
}

impl TryFrom<&JsonValue> for Instrument {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value["type"]
            .as_str()
            .context(err_field("type", "string"))?
        {
            "sample" => Ok(Instrument::Sample {
                data: WavReader::open(
                    value["path"]
                        .as_str()
                        .context(err_field("path", "string"))?,
                )
                .context("can't open sample file")?
                .samples()
                .map(|result| result.unwrap())
                .collect(),
                loops: value["loops"].as_bool().unwrap_or(false),
                resets: value["resets"].as_bool().unwrap_or(false),
            }),
            "expression" => Ok(Instrument::Expression {
                expr: Expr::from_str(
                    value["expression"]
                        .as_str()
                        .context(err_field("expression", "string"))?,
                )
                .context("can't parse expression")?,
                resets: value["resets"].as_bool().unwrap_or(false),
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
                .as_u16()
                .context(err_field("tuning", "unsigned 16-bit integer"))?,
            mask: Mask::try_from(value)?,
        })
    }
}

/// From the string mask and a string of allowed notes
impl TryFrom<&JsonValue> for Mask {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        Ok(Mask(
            many0(preceded(
                space0,
                alt((
                    note(
                        value["notes"]
                            .as_str()
                            .context("the \"notes\" field is not a string")?,
                    ),
                    rest(),
                    length(),
                    octave(),
                    volume(),
                )),
            ))(
                value["mask"]
                    .as_str()
                    .context("the \"mask\" field is not a string")?,
            )
            .map_err(|e| e.to_owned())
            .context("cannot parse mask input")?
            .1,
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
                .map(move |(name, chan)| -> anyhow::Result<(String, Channel)> {
                    Ok((name.to_string(), chan.try_into()?))
                })
                .collect::<anyhow::Result<HashMap<String, Channel>>>()?,
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
                .map(move |(name, track)| -> anyhow::Result<(String, Track)> {
                    Ok((name.to_string(), track.try_into()?))
                })
                .collect::<anyhow::Result<HashMap<String, Track>>>()?,
        ))
    }
}

impl TryFrom<&JsonValue> for Root {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        Ok(Root(
            value
                .entries()
                .map(move |(name, album)| -> anyhow::Result<(String, Album)> {
                    Ok((name.to_string(), album.try_into()?))
                })
                .collect::<anyhow::Result<HashMap<String, Album>>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use json::object;

    use super::*;
    #[test]
    pub fn note_parser() {
        assert_eq!(
            ("iueg", MaskAtoms::Note(2)),
            note("abcde")("ciueg").unwrap()
        );
    }
    #[test]
    pub fn length_parser() {
        assert_eq!(
            ("iueg", MaskAtoms::Length(16)),
            length()("$16iueg").unwrap()
        );
    }
    #[test]
    pub fn octave_parser() {
        assert_eq!(("iueg", MaskAtoms::Octave(4)), octave()("@4iueg").unwrap());
    }
    #[test]
    pub fn rest_parser() {
        assert_eq!(("iueg", MaskAtoms::Rest), rest()(".iueg").unwrap());
    }
    #[test]
    pub fn mask_parser() {
        assert_eq!(
            Mask(vec![
                MaskAtoms::Octave(4),
                MaskAtoms::Length(4),
                MaskAtoms::Volume(100),
                MaskAtoms::Note(0),
                MaskAtoms::Rest,
                MaskAtoms::Note(3),
                MaskAtoms::Note(5)
            ]),
            Mask::try_from(&object! {
                "instrument": "piano-sample",
                "notes": "aAbcCdDefFgG",
                "tuning": 442,
                "mask": "@4$4!100 a.cd"
            })
            .unwrap()
        );
    }
    #[test]
    pub fn instrument_parser() {
        assert_eq!(
            Instrument::Expression {
                expr: Expr::from_str("sin(x)").unwrap(),
                resets: true
            },
            Instrument::try_from(&object! {
                "type": "expression",
                "expression": "sin(x)",
                "resets": true
            })
            .unwrap()
        );
    }
    #[test]
    pub fn channel_parser() {
        assert_eq!(
            Channel {
                instrument: Instrument::Expression {
                    expr: Expr::from_str("sin(x)").unwrap(),
                    resets: true
                },
                tuning: 442,
                mask: Mask(vec![
                    MaskAtoms::Octave(4),
                    MaskAtoms::Length(4),
                    MaskAtoms::Volume(100),
                    MaskAtoms::Note(0),
                    MaskAtoms::Rest,
                    MaskAtoms::Note(3),
                    MaskAtoms::Note(5)
                ])
            },
            Channel::try_from(&object! {
                "instrument": {
                    "type": "expression",
                    "expression": "sin(x)",
                    "resets": true
                },
                "notes": "aAbcCdDefFgG",
                "tuning": 442,
                "mask": "@4$4!100 a.cd"
            })
            .unwrap()
        );
    }
}
