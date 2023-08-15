use anyhow::{Context, Error as AError, Ok as AOk, Result as AResult};
use derive_new::new;
use json::JsonValue;
use logos::{Lexer, Logos, Skip};
use meval::Expr;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::NonZeroU8;
use std::str::FromStr;
use text_lines::TextLines as TextPosition;
mod parsing_errors;
use parsing_errors::ParseError as PError;

pub(crate) const SAMPLE_RATE: u32 = 48000;

#[derive(PartialEq, Debug)]
pub(crate) struct Root(pub(crate) HashMap<String, Album>);

#[derive(new, PartialEq, Debug)]
pub(crate) struct Album {
    pub(crate) artist: String,
    pub(crate) tracks: HashMap<String, Track>,
}

#[derive(new, PartialEq, Debug)]
pub(crate) struct Track {
    pub(crate) bpm: u16,
    pub(crate) channels: HashMap<String, Channel>,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Instrument {
    Expression { expr: Expr },
}

impl Instrument {
    pub(crate) fn gen(
        &self,
        notes: u8,
        tuning: f32,
    ) -> AResult<impl Fn(usize, u8, u8, u8) -> Vec<f32>> {
        match self {
            Self::Expression { expr } => {
                let func = expr.clone().bind2("t", "f")?;
                AOk(
                    move |len: usize, n: u8, octave: u8, volume: u8| -> Vec<f32> {
                        let f = (tuning as f64 / 16f64)
                            * 2.0_f64.powf(((notes * octave + n) as f64) / (notes as f64));
                        (1..len)
                            .map(|i| {
                                let t = (i as f64) / (SAMPLE_RATE as f64);
                                (func(t, f) * ((volume as f64) / 100f64)) as f32
                            })
                            .collect()
                    },
                )
            }
        }
    }
}

#[derive(PartialEq, Debug, new)]
pub(crate) struct Channel {
    pub(crate) instrument: Instrument,
    pub(crate) tuning: f32,
    pub(crate) mask: Mask,
}

#[derive(PartialEq, Debug, Logos)]
#[logos(extras = Extras)]
#[logos(error = PError)]
pub(crate) enum MaskAtom {
    #[regex(r"@[0-9]{2}", octave)]
    Octave(NonZeroU8),
    Length(u8),
    Volume(u8),
    Note(u8),
    #[token(".")]
    Rest,
    #[regex(r"[ \t\n\f\r]+", junk)]
    Dummy,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Mask(pub(crate) u8, pub(crate) Vec<MaskAtom>);

pub(crate) struct Extras {
    notes: String,
    position: TextPosition,
    index: usize,
}

impl Extras {
    fn new(notes: String, position: TextPosition) -> Self {
        Extras {
            notes: notes,
            position: position,
            index: 0,
        }
    }
}
fn increment(lex: &mut Lexer<MaskAtom>) {
    lex.extras.index += lex.slice().chars().count();
}

fn junk(lex: &mut Lexer<MaskAtom>) -> Skip {
    increment(lex);
    Skip
}

fn octave(lex: &mut Lexer<MaskAtom>) -> Result<NonZeroU8, PError> {
    Ok(NonZeroU8::new(7).unwrap())
}

fn err_field(field: &str, r#type: &str) -> String {
    format!("field \"{}\" is not a \"{}\"", field, r#type)
}

impl TryFrom<&JsonValue> for Instrument {
    type Error = AError;
    fn try_from(value: &JsonValue) -> AResult<Self, Self::Error> {
        match value.entries().next().context("empty instrument")?.0 {
            "expr" => AOk(Instrument::Expression {
                expr: Expr::from_str(
                    value["expr"]
                        .as_str()
                        .context(err_field("expr", "string"))?,
                )
                .context("invalid instrument expression")?,
            }),
            _ => Err(AError::msg("unknown instrument type")),
        }
    }
}

impl TryFrom<&JsonValue> for Channel {
    type Error = AError;
    fn try_from(value: &JsonValue) -> AResult<Self, AError> {
        AOk(Channel {
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
    type Error = AError;
    fn try_from(value: &JsonValue) -> AResult<Self, Self::Error> {
        let notes = value["notes"]
            .as_str()
            .context("the \"notes\" field is not a string")?;
        AOk(Mask(
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
    type Error = AError;
    fn try_from(value: &JsonValue) -> AResult<Self, Self::Error> {
        AOk(Track::new(
            value["BPM"].as_u16().context(err_field("BPM", "u16"))?,
            value["channels"]
                .entries()
                .map(move |(name, chan)| -> AResult<(String, Channel)> {
                    AOk((name.to_string(), chan.try_into()?))
                })
                .collect::<AResult<HashMap<String, Channel>>>()?,
        ))
    }
}

impl TryFrom<&JsonValue> for Album {
    type Error = AError;
    fn try_from(value: &JsonValue) -> AResult<Self, Self::Error> {
        AOk(Album::new(
            value["artist"]
                .as_str()
                .context(err_field("artist", "string"))?
                .to_string(),
            value["tracks"]
                .entries()
                .map(move |(name, track)| -> AResult<(String, Track)> {
                    AOk((name.to_string(), track.try_into()?))
                })
                .collect::<AResult<HashMap<String, Track>>>()?,
        ))
    }
}

impl TryFrom<JsonValue> for Root {
    type Error = AError;
    fn try_from(value: JsonValue) -> AResult<Self, Self::Error> {
        AOk(Root(
            value
                .entries()
                .map(move |(name, album)| -> AResult<(String, Album)> {
                    AOk((name.to_string(), album.try_into()?))
                })
                .collect::<AResult<HashMap<String, Album>>>()?,
        ))
    }
}

#[test]
fn mask_parser() {
    assert_eq!(
        Mask(
            12,
            vec![
                MaskAtom::Octave(std::num::NonZeroU8::new(4).unwrap()),
                MaskAtom::Length(4),
                MaskAtom::Volume(100),
                MaskAtom::Note(0),
                MaskAtom::Rest,
                MaskAtom::Note(3),
                MaskAtom::Note(5)
            ]
        ),
        Mask::try_from(&json::object! {
            "instrument": "piano-sample",
            "notes": "aAbcCdDefFgG",
            "tuning": 442,
            "mask": "@4$4!100 a.cd"
        })
        .unwrap()
    );
}
#[test]
fn instrument_parser() {
    assert_eq!(
        Instrument::Expression {
            expr: Expr::from_str("sin(2*pi*f*t)").unwrap()
        },
        Instrument::try_from(&json::object! {
            "expr": "sin(2*pi*f*t)"
        })
        .unwrap()
    );
}
#[test]
fn channel_parser() {
    assert_eq!(
        Channel {
            instrument: Instrument::Expression {
                expr: Expr::from_str("sin(2*pi*f*x)").unwrap()
            },
            tuning: 442f32,
            mask: Mask(
                12,
                vec![
                    MaskAtom::Octave(std::num::NonZeroU8::new(4).unwrap()),
                    MaskAtom::Length(4),
                    MaskAtom::Volume(100),
                    MaskAtom::Note(0),
                    MaskAtom::Rest,
                    MaskAtom::Note(3),
                    MaskAtom::Note(5)
                ]
            )
        },
        Channel::try_from(&json::object! {
            "instrument": {
                "expr": "sin(2*pi*f*x)"
            },
            "notes": "aAbcCdDefFgG",
            "tuning": 442,
            "mask": "@4$4!100 a.cd"
        })
        .unwrap()
    );
}
#[test]
fn root_parser() {
    assert_eq!(
        Root(HashMap::from([(
            "My First Album".to_string(),
            Album::try_from(&json::object! {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 60,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1",
                                    "resets": true
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 .a"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)",
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
        Root::try_from(json::object! {
            "My First Album": {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 60,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1",
                                    "resets": true
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 .a"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)",
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
#[test]
fn bpm_test() {
    assert_eq!(
        Root::try_from(json::object! {
            "My First Album": {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 60,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 ab"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 ab"
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
        .mix()
        .unwrap()
        .get("My First Album")
        .unwrap()
        .1
        .get("My First Song")
        .unwrap()
        .len()
            + 1,
        Root::try_from(json::object! {
            "My First Album": {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 120,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 a"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 a"
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
        .mix()
        .unwrap()
        .get("My First Album")
        .unwrap()
        .1
        .get("My First Song")
        .unwrap()
        .len()
    )
}
#[test]
fn note_loss_test() {
    assert_eq!(
        Root::try_from(json::object! {
            "My First Album": {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 60,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 ab"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 a"
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
        .mix()
        .unwrap()
        .get("My First Album")
        .unwrap()
        .1
        .get("My First Song")
        .unwrap()
        .len()
            + 1,
        Root::try_from(json::object! {
            "My First Album": {
                "artist": "me",
                "tracks": {
                    "My First Song": {
                        "BPM": 120,
                        "channels": {
                            "piano": {
                                "instrument": {
                                    "expr": "4*abs(f*t-floor(f*t+1/2))-1"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 a"
                            },
                            "synth": {
                                "instrument": {
                                    "expr": "sin(2*pi*f*t)"
                                },
                                "notes": "aAbcCdDefFgG",
                                "tuning": 442,
                                "mask": "@4$4!100 a"
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
        .mix()
        .unwrap()
        .get("My First Album")
        .unwrap()
        .1
        .get("My First Song")
        .unwrap()
        .len()
    )
}
