use anyhow::{Ok, Result};
use derive_new::new;
use meval::Expr;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

pub(crate) use self::parsing::MaskAtom;

mod parsing;

pub(crate) const SAMPLE_RATE: u32 = 48000;

#[derive(PartialEq, Debug, Serialize)]
pub(crate) struct Root(pub(crate) HashMap<String, Album>);

#[derive(new, PartialEq, Debug, Serialize)]
pub(crate) struct Album {
    pub(crate) artist: String,
    pub(crate) tracks: HashMap<String, Track>,
}

#[derive(new, PartialEq, Debug, Serialize)]
pub(crate) struct Track {
    pub(crate) bpm: u16,
    pub(crate) channels: HashMap<String, Channel>,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Instrument {
    Expression { expr: Expr },
}

impl Serialize for Instrument {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Expression { expr } => serializer.serialize_str(format!("{:?}", expr).as_str()),
        }
    }
}

impl Instrument {
    pub(crate) fn gen(
        &self,
        notes: u8,
        tuning: f32,
    ) -> Result<impl Fn(usize, u8, u8, u8) -> Vec<f32>> {
        match self {
            Self::Expression { expr } => {
                let func = expr.clone().bind2("t", "f")?;
                Ok(
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

#[derive(PartialEq, Debug, new, Serialize)]
pub(crate) struct Channel {
    pub(crate) instrument: Instrument,
    pub(crate) tuning: f32,
    pub(crate) mask: Mask,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Mask(pub(crate) u8, pub(crate) Vec<MaskAtom>);
