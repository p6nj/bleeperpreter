pub use self::de::Signal;
use anyhow::{Ok, Result};
use bppt::Notes;
use derive_new::new;
use serde::Deserialize;
use std::fmt::Debug;
use std::num::{NonZeroU16, NonZeroUsize};

mod de;
mod default;

pub(crate) const SAMPLE_RATE: u32 = 48000;

#[derive(new, PartialEq, Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "BPM")]
    pub bpm: NonZeroU16,
    pub channels: Vec<Channel>,
}

impl Channel {
    pub(crate) fn generator(&self) -> Result<impl Fn(NonZeroUsize, u8, u8, u8) -> Vec<f32>> {
        let func = self
            .signal
            .clone()
            .0
            .bind2("t", "f")
            .expect("unknown variable in the expression");
        let notes = self.notes.set;
        let tuning = self.tuning;
        Ok(
            move |len: NonZeroUsize, n: u8, octave: u8, volume: u8| -> Vec<f32> {
                let f = (tuning as f64 / 16f64)
                    * 2.0_f64.powf(((notes * octave + n) as f64) / (notes as f64));
                (1..=usize::from(len))
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
pub struct Channel {
    pub signal: Signal,
    #[serde(flatten)]
    pub notes: Notes,
    pub tuning: f32,
}
