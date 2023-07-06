use hound::WavReader;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::str::FromStr;

use anyhow::{Context, Error};
use json::JsonValue;
use meval::{tokenizer::Operation, Expr};
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::multi::many0;
use nom::{character::complete::one_of, combinator::map_res, sequence::preceded, IResult};

pub struct Album {
    name: String,
    artist: String,
    tracks: HashMap<String, Track>,
}

struct Track {
    bpm: u16,
    instruments: HashMap<String, Instrument>,
    effects: HashMap<String, Effect>,
    channels: HashMap<String, Channel>,
}

#[derive(PartialEq, Debug)]
enum Instrument {
    Sample {
        data: Vec<f32>,
        r#loops: bool,
        resets: bool,
    },
    Expression {
        expr: Expr,
        resets: bool,
    },
    Mix {
        first: Box<Instrument>,
        second: Box<Instrument>,
        operator: Operation,
    },
}

// TODO: complete effect structure
#[derive(PartialEq, Debug)]
struct Effect {
    base: String,
    mask: Mask,
}

#[derive(PartialEq, Debug)]
struct Channel {
    instrument: Instrument,
    effects: Vec<Effect>,
    tuning: u16,
    mask: Mask,
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
struct Mask(Vec<MaskAtoms>);

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(one_of(notes), move |c| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Note(
            notes
                .find(c)
                .with_context(|| format!("unknown note: {}", c))?
                .try_into()
                .context("converting note index to u8 (shouldn't fail)")?,
        ))
    })
}

fn rest<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(char('.'), move |_| Ok::<MaskAtoms, Error>(MaskAtoms::Rest))
}

fn length<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('$'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Length(n))
    })
}

fn octave<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('@'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Octave(n))
    })
}

fn volume<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('!'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Volume(n))
    })
}

fn err_field(field: &str, r#type: &str) -> String {
    format!("field \"{}\" is not a \"{}\"", field, r#type)
}

fn err_parse(r#type: &str, name: &str) -> String {
    format!("can't parse {} \"{}\"", r#type, name)
}

impl TryFrom<&JsonValue> for Effect {
    type Error = Error;
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        panic!("{}", value);
    }
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
                .samples::<f32>()
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

impl Channel {
    fn try_from(song_root: &JsonValue, target: &str) -> Result<Self, Error> {
        Ok(Channel {
            instrument: Instrument::try_from(
                &song_root["instruments"][song_root["channels"][target]["instrument"]
                    .as_str()
                    .context(err_field("instrument", "string"))?],
            )?,
            effects: {
                let mut vec = vec![];
                song_root["channels"][target]["effects"]
                    .members()
                    .try_for_each(|ef| {
                        Ok::<(), Error>(
                            vec.push(
                                Effect::try_from(
                                    &song_root["effects"][ef
                                        .as_str()
                                        .context(err_field("effects", "pure string array"))?],
                                )
                                .context(err_parse("effect", ef.as_str().unwrap()))?,
                            ),
                        )
                    })?;
                vec
            },
            tuning: song_root["channels"][target]["tuning"]
                .as_u16()
                .context(err_field("tuning", "unsigned 16-bit integer"))?,
            mask: Mask::try_from(&song_root["channels"][target])?,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use json::object;

    use super::*;
    #[test]
    pub fn note_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Note(2))), note("abcde")("ciueg"));
    }
    #[test]
    pub fn length_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Length(16))), length()("$16iueg"));
    }
    #[test]
    pub fn octave_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Octave(4))), octave()("@4iueg"));
    }
    #[test]
    pub fn rest_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Rest)), rest()(".iueg"));
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
                "effects": [
                    "low reverb"
                ],
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
                effects: vec![],
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
            Channel::try_from(
                &object! {"BPM": 60,
                "instruments": {
                    "piano-sample": {
                        "type": "sample",
                        "path": "sound/piano.wav",
                        "loop": false
                    },
                    "sine": {
                        "type": "expression",
                        "expression": "sin(x)",
                        "resets": true
                    },
                    "the two": {
                        "type": "mix",
                        "first": "piano-sample",
                        "second": "sine",
                        "operator": "+"
                    }
                },
                "effects": {
                    "low reverb": {
                        "base": "reverb",
                        "mask": "!26 ..a."
                    },
                    "piano env": {}
                },
                "channels": {
                    "piano": {
                        "instrument": "piano-sample",
                        "effects": [
                            "low reverb"
                        ],
                        "notes": "aAbcCdDefFgG",
                        "tuning": 442,
                        "mask": "@4$4!100 a.cd"
                    },
                    "synth": {
                        "instrument": "sine",
                        "notes": "aAbcCdDefFgG",
                        "tuning": 442,
                        "mask": "@4$4!100 a.cd"
                    }
                }},
                "piano"
            )
            .unwrap()
        );
    }
}
