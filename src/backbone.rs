use anyhow::{Context, Error, Ok};
use derive_new::new;
use json::JsonValue;
use meval::Expr;
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::multi::many0;
use nom::{character::complete::one_of, combinator::map_res, sequence::preceded, IResult};
use rodio::Decoder;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Root(HashMap<String, Album>);

#[derive(new, PartialEq, Debug)]
pub struct Album {
    pub artist: String,
    pub tracks: HashMap<String, Track>,
}

#[derive(new, PartialEq, Debug)]
pub struct Track {
    pub bpm: u16,
    pub channels: HashMap<String, Channel>,
}

pub enum Instrument {
    Sample {
        wav: Decoder<BufReader<File>>,
        loops: bool,
        resets: bool,
    },
    Expression {
        expr: Expr,
        resets: bool,
    },
}

impl PartialEq for Instrument {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Instrument::Sample {
                wav: _,
                loops,
                resets,
            } => match other {
                Instrument::Sample {
                    wav: _,
                    loops: other_loops,
                    resets: other_resets,
                } => loops == other_loops && resets == other_resets,
                Instrument::Expression { expr: _, resets: _ } => false,
            },
            Instrument::Expression { expr, resets } => match other {
                Instrument::Sample {
                    wav: _,
                    loops: _,
                    resets: _,
                } => false,
                Instrument::Expression {
                    expr: other_expr,
                    resets: other_resets,
                } => expr == other_expr && resets == other_resets,
            },
        }
    }
}

impl Debug for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instrument::Sample {
                wav: _,
                loops,
                resets,
            } => write!(
                f,
                "Sample {{ wav: ({:?}), loops: {:?}, resets: {:?} }}",
                "can't display", loops, resets
            ),
            Instrument::Expression { expr, resets } => {
                write!(f, "Expression {{ expr: {:?}, resets: {:?} }}", expr, resets)
            }
        }
    }
}

#[derive(PartialEq, Debug, new)]
pub struct Channel {
    pub instrument: Instrument,
    pub tuning: u16,
    pub mask: Mask,
}

#[derive(PartialEq, Debug)]
pub enum MaskAtom {
    Octave(u8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

#[derive(Debug, PartialEq)]
pub struct Mask(pub Vec<MaskAtom>);

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(one_of(notes), move |c| {
        Ok::<MaskAtom>(MaskAtom::Note(
            notes
                .find(c)
                .with_context(|| format!("unknown note: {}", c))?
                .try_into()
                .context("converting note index to u8 (shouldn't fail)")?,
        ))
    })
}

fn rest<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(char('.'), move |_| Ok::<MaskAtom>(MaskAtom::Rest))
}

fn length<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(preceded(char('$'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtom>(MaskAtom::Length(n))
    })
}

fn octave<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(preceded(char('@'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtom>(MaskAtom::Octave(n))
    })
}

fn volume<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(preceded(char('!'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtom>(MaskAtom::Volume(n))
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
                wav: Decoder::new(BufReader::new(File::open(
                    value["path"]
                        .as_str()
                        .context(err_field("path", "string"))?,
                )?))?,
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

impl TryFrom<JsonValue> for Root {
    type Error = Error;
    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
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
        assert_eq!(("iueg", MaskAtom::Note(2)), note("abcde")("ciueg").unwrap());
    }
    #[test]
    pub fn length_parser() {
        assert_eq!(("iueg", MaskAtom::Length(16)), length()("$16iueg").unwrap());
    }
    #[test]
    pub fn octave_parser() {
        assert_eq!(("iueg", MaskAtom::Octave(4)), octave()("@4iueg").unwrap());
    }
    #[test]
    pub fn rest_parser() {
        assert_eq!(("iueg", MaskAtom::Rest), rest()(".iueg").unwrap());
    }
    #[test]
    pub fn mask_parser() {
        assert_eq!(
            Mask(vec![
                MaskAtom::Octave(4),
                MaskAtom::Length(4),
                MaskAtom::Volume(100),
                MaskAtom::Note(0),
                MaskAtom::Rest,
                MaskAtom::Note(3),
                MaskAtom::Note(5)
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
                    MaskAtom::Octave(4),
                    MaskAtom::Length(4),
                    MaskAtom::Volume(100),
                    MaskAtom::Note(0),
                    MaskAtom::Rest,
                    MaskAtom::Note(3),
                    MaskAtom::Note(5)
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
    #[test]
    pub fn root_parser() {
        assert_eq!(
            Root(HashMap::from([(
                "My First Album".to_string(),
                Album::try_from(&object! {
                    "artist": "me",
                    "tracks": {
                        "My First Song": {
                            "BPM": 60,
                            "channels": {
                                "piano": {
                                    "instrument": {
                                        "type": "sample",
                                        "path": "sound/piano.wav",
                                        "loops": false
                                    },
                                    "effects": [
                                        "low reverb"
                                    ],
                                    "notes": "aAbcCdDefFgG",
                                    "tuning": 442,
                                    "mask": "@4$4!100 a.cd"
                                },
                                "synth": {
                                    "instrument": {
                                        "type": "expression",
                                        "expression": "sin(x)",
                                        "resets": true
                                    },
                                    "notes": "aAbcCdDefFgG",
                                    "tuning": 442,
                                    "mask": "@4$4!100 a.cd"
                                }
                            }
                        }
                    }
                })
                .unwrap()
            )])),
            Root::try_from(object! {
                "My First Album": {
                    "artist": "me",
                    "tracks": {
                        "My First Song": {
                            "BPM": 60,
                            "channels": {
                                "piano": {
                                    "instrument": {
                                        "type": "sample",
                                        "path": "sound/piano.wav",
                                        "loops": false
                                    },
                                    "effects": [
                                        "low reverb"
                                    ],
                                    "notes": "aAbcCdDefFgG",
                                    "tuning": 442,
                                    "mask": "@4$4!100 a.cd"
                                },
                                "synth": {
                                    "instrument": {
                                        "type": "expression",
                                        "expression": "sin(x)",
                                        "resets": true
                                    },
                                    "notes": "aAbcCdDefFgG",
                                    "tuning": 442,
                                    "mask": "@4$4!100 a.cd"
                                }
                            }
                        }
                    }
                }
            })
            .unwrap()
        );
    }
}
