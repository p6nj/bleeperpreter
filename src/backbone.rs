use anyhow::{Context, Error, Ok, Result};
use audio_processor_analysis::window_functions::make_hann_vec;
use derive_new::new;
use json::JsonValue;
use meval::Expr;
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::multi::many0;
use nom::{character::complete::one_of, combinator::map_res, sequence::preceded, IResult};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rodio::{Decoder, Source};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::num::NonZeroU8;
use std::str::FromStr;

mod resampling;
use resampling::resample;

mod pitch_shift;
use pitch_shift::PitchShifter;

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
    Expression { expr: Expr, fmod: Expr },
}

impl Instrument {
    pub(crate) fn gen(
        &self,
        notes: u8,
        tuning: f32,
    ) -> Result<impl Fn(usize, u8, u8, u8) -> Vec<f32>> {
        match self {
            Self::Expression { expr, fmod } => {
                let func = expr.clone().bind2("t", "f")?;
                let fmod = fmod.clone().bind2("t", "f")?;
                Ok(
                    move |len: usize, n: u8, octave: u8, volume: u8| -> Vec<f32> {
                        (1..len)
                            .map(|i| {
                                let t = (i as f64) / (SAMPLE_RATE as f64);
                                let f = (tuning as f64 / 16f64)
                                    * 2.0_f64.powf(((notes * octave + n) as f64) / (notes as f64));
                                let f = fmod(t, f);
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

#[derive(PartialEq, Debug)]
pub(crate) enum MaskAtom {
    Octave(NonZeroU8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Mask(pub(crate) u8, pub(crate) Vec<MaskAtom>);

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
                fmod: Expr::from_str(value["fmod"].as_str().unwrap_or("f"))
                    .context("invalid fmod expression")?,
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
            many0(preceded(
                space0,
                alt((note(notes), rest(), length(), octave(), volume())),
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
    pub(crate) fn note_parser() {
        assert_eq!(("iueg", MaskAtom::Note(2)), note("abcde")("ciueg").unwrap());
    }
    #[test]
    pub(crate) fn length_parser() {
        assert_eq!(("iueg", MaskAtom::Length(16)), length()("$16iueg").unwrap());
    }
    #[test]
    pub(crate) fn octave_parser() {
        assert_eq!(
            ("iueg", MaskAtom::Octave(NonZeroU8::new(4).unwrap())),
            octave()("@4iueg").unwrap()
        );
    }
    #[test]
    pub(crate) fn rest_parser() {
        assert_eq!(("iueg", MaskAtom::Rest), rest()(".iueg").unwrap());
    }
    #[test]
    pub(crate) fn mask_parser() {
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
    pub(crate) fn instrument_parser() {
        assert_eq!(
            Instrument::Expression {
                expr: Expr::from_str("sin(2*pi*f*t)").unwrap(),
                fmod: Expr::from_str("f").unwrap()
            },
            Instrument::try_from(&object! {
                "expr": "sin(2*pi*f*t)"
            })
            .unwrap()
        );
    }
    #[test]
    pub(crate) fn channel_parser() {
        assert_eq!(
            Channel {
                instrument: Instrument::Expression {
                    expr: Expr::from_str("sin(2*pi*f*x)").unwrap(),
                    fmod: Expr::from_str("f").unwrap()
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
            Channel::try_from(&object! {
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
    pub(crate) fn root_parser() {
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
            Root::try_from(object! {
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
}
