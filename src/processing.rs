use std::{collections::HashMap, time::Duration};

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
    pub fn process(&self, bpm: &u16) -> Result<Samples> {
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
                let mut result = vec![];
                let mut shifter = PitchShifter::new(50, wav.sample_rate().try_into()?);
                self.mask.1.iter().try_for_each(|a| -> Result<()> {
                    match *a {
                        MaskAtom::Octave(o) => octave = o,
                        MaskAtom::Length(l) => length = l,
                        MaskAtom::Volume(v) => volume = v,
                        MaskAtom::Note(n) => {
                            let in_b = wav
                                .take_duration(Duration::from_secs_f64(
                                    (((length as u16) * bpm) as f64) / 240f64,
                                ))
                                .convert_samples()
                                .collect::<Vec<f32>>();
                            let mut out_b = vec![0.0; in_b.len()];
                            shifter.shift_pitch(
                                16,
                                n.into(),
                                self.mask.0.into(),
                                &mut in_b.as_slice(),
                                &mut out_b.as_slice(),
                            );
                            result.append(&mut out_b);
                        }
                        MaskAtom::Rest => result.append(&mut vec![
                            0f32;
                            ((((((length as u16) * bpm) as f64) / 240f64)
                                * (wav.sample_rate() as f64))
                                .trunc()
                                as u32).try_into()?
                        ]),
                    }
                    Ok(())
                })?;
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
