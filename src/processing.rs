use crate::backbone::{self, Instrument, MaskAtom};
use anyhow::Result;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::collections::HashMap;

type Track = HashMap<String, Samples>;
type Album = (String, HashMap<String, Track>);
type Samples = Vec<f32>;

type MixedAlbum = (String, HashMap<String, Samples>);
pub type MixedRoot = HashMap<String, MixedAlbum>;

impl backbone::Track {
    fn process(&mut self) -> Result<Track> {
        Ok(self
            .channels
            .par_iter_mut()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process(&self.bpm)?))
            })
            .collect::<Result<Track>>()?)
    }
}

impl backbone::Album {
    fn process(&mut self) -> Result<Album> {
        Ok((
            self.artist.clone(),
            self.tracks
                .iter_mut()
                .map(|(name, track)| -> Result<(String, Track)> {
                    Ok((name.clone(), track.process()?))
                })
                .collect::<Result<HashMap<String, Track>>>()?,
        ))
    }
    fn mix(&mut self) -> Result<MixedAlbum> {
        let processed = self.process()?;
        Ok((
            processed.0.clone(),
            processed
                .1
                .iter()
                .map(|(name, track)| {
                    (
                        name.clone(),
                        track
                            .par_iter()
                            .fold(
                                || Vec::<f32>::new(),
                                move |acc, (_, samples)| {
                                    samples
                                        .iter()
                                        .enumerate()
                                        .map(|(i, s)| {
                                            acc.get(i).unwrap_or(&0f32)
                                                + (*s / (track.len() as f32))
                                        })
                                        .collect()
                                },
                            )
                            .reduce(|| Vec::<f32>::new(), move |a, b| [a, b].concat()),
                    )
                })
                .collect::<HashMap<String, Samples>>(),
        ))
    }
}

impl backbone::Channel {
    fn process(&mut self, bpm: &u16) -> Result<Samples> {
        let sr = 48000u32;
        let mut octave = 4u8;
        let mut volume = 100u8;

        let mut result = vec![];
        let genlength = move |length: u8| -> Result<usize> {
            Ok(
                ((((((length as u16) * bpm) as f64) / 240f64) * (sr as f64)).trunc() as u32)
                    .try_into()?,
            )
        };
        let mut real_length = genlength(4)?;

        let expr = match &self.instrument {
            Instrument::Sample {
                data: _,
                loops: _,
                resets: _,
            } => false,
            Instrument::Expression { expr: _, resets: _ } => true,
        };
        let gensamples = self.instrument.gen(self.mask.0)?;
        match expr {
            true => {
                let gen = gensamples.0.unwrap();
                for a in self.mask.1.iter() {
                    match *a {
                        MaskAtom::Octave(o) => octave = u8::from(o) - 1,
                        MaskAtom::Length(l) => real_length = genlength(l)?,
                        MaskAtom::Volume(v) => volume = v,
                        MaskAtom::Note(n) => {
                            result.append(&mut gen(real_length, n, octave, volume))
                        }
                        MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
                    };
                }
            }
            false => {
                let mut gen = gensamples.1.unwrap();
                for a in self.mask.1.iter() {
                    match *a {
                        MaskAtom::Octave(o) => octave = u8::from(o) - 1,
                        MaskAtom::Length(l) => real_length = genlength(l)?,
                        MaskAtom::Volume(v) => volume = v,
                        MaskAtom::Note(n) => {
                            result.append(&mut gen(real_length, n, octave, volume))
                        }
                        MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
                    };
                }
            }
        }
        Ok(result)
    }
}

impl backbone::Root {
    pub fn mix(mut self) -> Result<MixedRoot> {
        Ok(self
            .0
            .iter_mut()
            .map(|(name, album)| -> Result<(String, MixedAlbum)> {
                Ok((name.clone(), album.mix()?))
            })
            .collect::<Result<MixedRoot>>()?)
    }
}
