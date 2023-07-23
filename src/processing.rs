use std::collections::HashMap;

mod pitch_shift;
use anyhow::Result;
use pitch_shift::PitchShifter;
use rodio::Source;

use crate::backbone::{self, MaskAtom};
type Track = HashMap<String, Samples>;
type Album = HashMap<String, Track>;
type Root = HashMap<String, Album>;
type Samples = Vec<f32>;
impl backbone::Track {
    pub fn process(&self) -> Result<Track> {
        Ok(self
            .channels
            .iter()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process()?))
            })
            .collect::<Result<Track>>()?)
    }
}
impl backbone::Album {
    pub fn process(&self) -> Album {
        Album::new()
    }
}
impl backbone::Channel {
    pub fn process(&self) -> Result<Samples> {
        let mut octave = 4u8;
        let mut length = 4u8;
        let mut volume = 100u8;
        match &self.instrument {
            backbone::Instrument::Sample {
                wav,
                r#loops,
                resets,
            } => {
                // tuning has to be done by stretching sound manually
                let result = vec![];
                let shifter = PitchShifter::new(50, wav.sample_rate().try_into()?);
                self.mask.0.iter().for_each(|a| match *a {
                    MaskAtom::Octave(o) => octave = o,
                    MaskAtom::Length(l) => length = l,
                    MaskAtom::Volume(v) => volume = v,
                    MaskAtom::Note(n) => todo!(),
                    MaskAtom::Rest => (result, vec![0.0; length]),
                });
                let mut out_b = vec![0.0; in_b.len()];
                shifter.shift_pitch(16, shift, in_b, out_b);
                Ok(result)
            }
            backbone::Instrument::Expression { expr, resets } => todo!(),
        }
    }
}
impl backbone::Root {
    pub fn process(&self) -> Root {
        Root::new()
    }
}
