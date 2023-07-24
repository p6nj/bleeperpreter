use std::{collections::HashMap, io::BufReader, time::Duration};
mod pitch_shift;
mod resampling;
use crate::backbone::{self, Instrument, MaskAtom};
use anyhow::Result;
use pitch_shift::PitchShifter;
use resampling::resample;
use rodio::{Decoder, Source};

type Track = HashMap<String, Samples>;
type Album = HashMap<String, Track>;
type Root = HashMap<String, Album>;
type Samples = Vec<f32>;
impl backbone::Track {
    pub fn process(&mut self) -> Result<Track> {
        Ok(self
            .channels
            .iter_mut()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process(&self.bpm)?))
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
    pub fn process(&mut self, bpm: &u16) -> Result<Samples> {
        let sr = 48000u32;
        let mut octave = 4u8;
        let mut length = 4u8;
        let mut volume = 100u8;

        let mut result = vec![];
        for a in self.mask.1.iter() {
            let genlength = move || -> Result<usize> {
                Ok(
                    ((((((length as u16) * bpm) as f64) / 240f64) * (sr as f64)).trunc() as u32)
                        .try_into()?,
                )
            };
            match *a {
                MaskAtom::Octave(o) => octave = o,
                MaskAtom::Length(l) => length = l,
                MaskAtom::Volume(v) => volume = v,
                MaskAtom::Note(n) => {
                    let len = genlength()?;
                    match &self.instrument {
                        Instrument::Sample { wav, loops, resets } => {
                            // tuning has to be done by stretching sound manually
                            let decoder = Decoder::new(BufReader::new(wav.try_clone()?))?;
                            let samples =
                                resample(decoder.convert_samples::<f32>().collect::<Vec<f32>>())?;
                            let mut shifter = PitchShifter::new(50, sr.try_into()?);
                            let mut in_b: Vec<f32> = match len > samples.len() {
                                true => samples
                                    .repeat((len - samples.len()) % samples.len())
                                    .split_at(len)
                                    .0
                                    .into(),
                                false => samples.split_at(len).0.into(),
                            };
                            let mut out_b = vec![0.0; in_b.len()];
                            shifter.shift_pitch(
                                16,
                                n.into(),
                                self.mask.0.into(),
                                &mut in_b,
                                &mut out_b,
                            );
                            result.append(&mut out_b);
                        }
                        Instrument::Expression { expr, resets } => todo!(),
                    }
                }
                MaskAtom::Rest => result.append(&mut vec![0f32; genlength()?]),
            }
        }
        Ok(result)
    }
}

impl backbone::Root {
    pub fn process(&self) -> Root {
        Root::new()
    }
}
