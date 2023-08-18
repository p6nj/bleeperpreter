use anyhow::{Ok, Result};
use derive_new::new;
use meval::Expr;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

pub(crate) use self::parsing::MaskAtom;

mod parsing;

pub(crate) const SAMPLE_RATE: u32 = 48000;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Root(pub(crate) HashMap<String, Album>);

#[derive(new, PartialEq, Debug, Deserialize)]
pub(crate) struct Album {
    pub(crate) artist: String,
    pub(crate) tracks: HashMap<String, Track>,
}

#[derive(new, PartialEq, Debug, Deserialize)]
pub(crate) struct Track {
    #[serde(rename = "BPM")]
    pub(crate) bpm: u16,
    pub(crate) channels: HashMap<String, Channel>,
}

#[derive(new, PartialEq, Debug)]
pub(crate) struct Notes {
    pub(crate) set: String,
    pub(crate) score: Mask,
}

impl Channel {
    pub(crate) fn generator(&self) -> Result<impl Fn(usize, u8, u8, u8) -> Vec<f32>> {
        let func = self.signal.clone().bind2("t", "f")?;
        let notes = self.notes.set.len() as u8;
        let tuning = self.tuning;
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

#[derive(PartialEq, Debug, new, Deserialize)]
pub(crate) struct Channel {
    pub(crate) signal: Expr,
    #[serde(flatten)]
    pub(crate) notes: Notes,
    pub(crate) tuning: f32,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Mask(pub(crate) Vec<MaskAtom>);
