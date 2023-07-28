use std::collections::HashMap;
mod error;
use crate::backbone::{self, Instrument, MaskAtom};
use anyhow::Result;

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
    pub fn process(&mut self) -> Result<Album> {
        Ok(self
            .tracks
            .iter_mut()
            .map(|(name, track)| -> Result<(String, Track)> {
                Ok((name.clone(), track.process()?))
            })
            .collect::<Result<Album>>()?)
    }
}

impl backbone::Channel {
    pub fn process(&mut self, bpm: &u16) -> Result<Samples> {
        let sr = 48000u32;
        let mut octave = 4u8;
        let mut length = 4u8;
        let mut volume = 100u8;
        let mut real_length = 0;

        let mut result = vec![];
        let genlength = move || -> Result<usize> {
            Ok(
                ((((((length as u16) * bpm) as f64) / 240f64) * (sr as f64)).trunc() as u32)
                    .try_into()?,
            )
        };
        let expr = match &self.instrument {
            Instrument::Sample {
                data,
                loops,
                resets,
            } => false,
            Instrument::Expression { expr, resets } => true,
        };
        let gensamples = self.instrument.gen(self.mask.0)?;
        match expr {
            true => {
                let gen = gensamples.0.unwrap();
                for a in self.mask.1.iter() {
                    match *a {
                        MaskAtom::Octave(o) => octave = o,
                        MaskAtom::Length(l) => length = l,
                        MaskAtom::Volume(v) => volume = v,
                        MaskAtom::Note(n) => result.append(&mut gen(real_length, n)),
                        MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
                    };
                    match *a {
                        MaskAtom::Length(_) | MaskAtom::Octave(_) | MaskAtom::Volume(_) => {
                            real_length = genlength()?
                        }
                        _ => {}
                    }
                }
            }
            false => {
                let mut gen = gensamples.1.unwrap();
                for a in self.mask.1.iter() {
                    match *a {
                        MaskAtom::Octave(o) => octave = o,
                        MaskAtom::Length(l) => length = l,
                        MaskAtom::Volume(v) => volume = v,
                        MaskAtom::Note(n) => result.append(&mut gen(real_length, n)),
                        MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
                    };
                    match *a {
                        MaskAtom::Length(_) | MaskAtom::Octave(_) | MaskAtom::Volume(_) => {
                            real_length = genlength()?
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(result)
    }
}

impl backbone::Root {
    pub fn process(&mut self) -> Result<Root> {
        Ok(self
            .0
            .iter_mut()
            .map(|(name, album)| -> Result<(String, Album)> {
                Ok((name.clone(), album.process()?))
            })
            .collect::<Result<Root>>()?)
    }
}
