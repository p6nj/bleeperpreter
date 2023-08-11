use crate::backbone::{self, MaskAtom};
use anyhow::Result;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use std::collections::HashMap;

type Track = HashMap<String, Samples>;
type Album = (String, HashMap<String, Track>);
type Samples = Vec<f32>;

type MixedAlbum = (String, HashMap<String, Samples>);
pub(crate) type MixedRoot = HashMap<String, MixedAlbum>;

impl backbone::Track {
    fn process(&mut self) -> Result<Track> {
        self.channels
            .par_iter_mut()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process(&self.bpm)?))
            })
            .collect::<Result<Track>>()
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
                            .iter()
                            .fold(Vec::<f32>::new(), move |acc, (_, samples)| {
                                samples
                                    .iter()
                                    .enumerate()
                                    .map(|(i, s)| {
                                        acc.get(i).unwrap_or(&0f32) + (*s / (track.len() as f32))
                                    })
                                    .collect()
                            }),
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

        let gen = self.instrument.gen(self.mask.0, self.tuning)?;
        self.mask.1.iter().try_for_each(|a| -> Result<()> {
            match *a {
                MaskAtom::Octave(o) => octave = u8::from(o) - 1,
                MaskAtom::Length(l) => real_length = genlength(l)?,
                MaskAtom::Volume(v) => volume = v,
                MaskAtom::Note(n) => result.append(&mut gen(real_length, n, octave, volume)),
                MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
            };
            Ok(())
        })?;
        Ok(result)
    }
}

impl backbone::Root {
    pub(crate) fn mix(mut self) -> Result<MixedRoot> {
        self.0
            .iter_mut()
            .map(|(name, album)| -> Result<(String, MixedAlbum)> {
                Ok((name.clone(), album.mix()?))
            })
            .collect::<Result<MixedRoot>>()
    }
}
