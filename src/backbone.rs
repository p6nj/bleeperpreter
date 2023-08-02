use anyhow::{Context, Error, Ok, Result};
use audio_processor_analysis::window_functions::{hann, make_hann_vec};
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

pub const SAMPLE_RATE: u32 = 48000;

#[derive(PartialEq, Debug)]
pub struct Root(pub HashMap<String, Album>);

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
        /// this vector of samples is already set to the project sample rate.
        data: Vec<f32>,
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
                data: _,
                loops,
                resets,
            } => match other {
                Instrument::Sample {
                    data: _,
                    loops: other_loops,
                    resets: other_resets,
                } => loops == other_loops && resets == other_resets,
                Instrument::Expression { expr: _, resets: _ } => false,
            },
            Instrument::Expression { expr, resets } => match other {
                Instrument::Sample {
                    data: _,
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

impl<'a> Instrument {
    pub fn gen(
        &'a self,
        notes: u8,
        tuning: f32,
    ) -> Result<(
        Option<impl Fn(usize, u8, u8, u8) -> Vec<f32>>,
        Option<impl FnMut(usize, u8, u8, u8) -> Vec<f32> + 'a>,
    )> {
        Ok(match self {
            Self::Expression { expr, resets } => {
                let func = expr.clone().bind2("t", "f")?;
                (
                    Some(
                        move |len: usize, n: u8, octave: u8, volume: u8| -> Vec<f32> {
                            (1..len)
                                .map(|i| {
                                    (func(
                                        (i as f64) / (SAMPLE_RATE as f64),
                                        (tuning as f64 / 16f64)
                                            * 2.0_f64.powf(
                                                ((notes * octave + n) as f64) / (notes as f64),
                                            ),
                                    ) * ((volume as f64) / 100f64))
                                        as f32
                                })
                                .collect()
                        },
                    ),
                    None,
                )
            }
            Self::Sample {
                data,
                loops,
                resets,
            } => {
                let mut shifter = PitchShifter::new(50, SAMPLE_RATE as usize);
                let sample_tuning = samples_fft_to_spectrum(
                    &{
                        let mut data = data
                            .par_iter()
                            .zip(make_hann_vec::<f32>(data.len()))
                            .map(|(a, b)| a * b)
                            .collect::<Vec<f32>>();
                        data.truncate(2usize.pow(((data.len() as f64).ln() / 2f64.ln()) as u32));
                        data
                    },
                    SAMPLE_RATE,
                    FrequencyLimit::Range(100f32, 1_000f32),
                    None,
                )
                .unwrap()
                .max()
                .0
                .val();
                (
                    None,
                    Some(
                        move |len: usize, n: u8, octave: u8, volume: u8| -> Vec<f32> {
                            let data = data.clone();
                            let in_b: Vec<f32> = match len > data.len() {
                                true => match loops {
                                    true => data
                                        .repeat((len - data.len()) % data.len())
                                        .split_at(len)
                                        .0
                                        .into(),
                                    false => {
                                        let slen = data.len();
                                        [data, vec![0f32; len - slen]].concat()
                                    }
                                },
                                false => data.split_at(len).0.into(),
                            };
                            // return in_b;
                            let mut out_b = vec![0.0; in_b.len()];
                            shifter.shift_pitch(
                                16,
                                sample_tuning
                                    - (tuning * 2.0_f32.powf((n as f32) / (notes as f32))),
                                &mut in_b
                                    .par_iter()
                                    .map(|sample| sample * ((volume as f32) / 100f32))
                                    .collect::<Vec<f32>>(),
                                &mut out_b,
                            );
                            out_b
                        },
                    ),
                )
            }
        })
    }
}

impl Debug for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instrument::Sample {
                data: _,
                loops,
                resets,
            } => write!(
                f,
                "Sample {{ data: ({}), loops: {:?}, resets: {:?} }}",
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
    pub tuning: f32,
    pub mask: Mask,
}

#[derive(PartialEq, Debug)]
pub enum MaskAtom {
    Octave(NonZeroU8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

#[derive(Debug, PartialEq)]
pub struct Mask(pub u8, pub Vec<MaskAtom>);

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
        match value["type"]
            .as_str()
            .context(err_field("type", "string"))?
        {
            "sample" => Ok(Instrument::Sample {
                data: {
                    let decoder = Decoder::new(BufReader::new(File::open(
                        value["path"]
                            .as_str()
                            .context(err_field("path", "string"))?,
                    )?))?;
                    {
                        let from_sr = decoder.sample_rate();
                        let channels = decoder.channels();
                        resample(
                            decoder
                                .convert_samples::<f32>()
                                .enumerate()
                                .filter(|(i, _)| i % (channels as usize) == 0)
                                .map(move |(_, e)| e)
                                .collect(),
                            from_sr.into(),
                            SAMPLE_RATE as f64,
                        )?
                    }
                },
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
    pub fn note_parser() {
        assert_eq!(("iueg", MaskAtom::Note(2)), note("abcde")("ciueg").unwrap());
    }
    #[test]
    pub fn length_parser() {
        assert_eq!(("iueg", MaskAtom::Length(16)), length()("$16iueg").unwrap());
    }
    #[test]
    pub fn octave_parser() {
        assert_eq!(
            ("iueg", MaskAtom::Octave(NonZeroU8::new(4).unwrap())),
            octave()("@4iueg").unwrap()
        );
    }
    #[test]
    pub fn rest_parser() {
        assert_eq!(("iueg", MaskAtom::Rest), rest()(".iueg").unwrap());
    }
    #[test]
    pub fn mask_parser() {
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
    pub fn instrument_parser() {
        assert_eq!(
            Instrument::Expression {
                expr: Expr::from_str("sin(2*pi*f*t)").unwrap(),
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
                    expr: Expr::from_str("sin(2*pi*n*x)").unwrap(),
                    resets: true
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
                                        "path": "sound/piano.data",
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
                                        "path": "sound/piano.data",
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
