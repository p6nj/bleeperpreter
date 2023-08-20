use crate::structure::{self, MaskAtom};
use anyhow::Result;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use std::collections::HashMap;

type Track = HashMap<String, Samples>;
type Album = (String, HashMap<String, Track>);
type Samples = Vec<f32>;

type MixedAlbum = (String, HashMap<String, Samples>);
pub(crate) type MixedRoot = HashMap<String, MixedAlbum>;

mod processing;
#[cfg(test)]
mod tests;

impl structure::Root {
    pub(crate) fn mix(mut self) -> Result<MixedRoot> {
        self.0
            .iter_mut()
            .map(|(name, album)| -> Result<(String, MixedAlbum)> {
                Ok((name.clone(), album.mix()?))
            })
            .collect::<Result<MixedRoot>>()
    }
}

impl structure::Album {
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
                        track.iter().fold(
                            Vec::<f32>::with_capacity(track.iter().next().unwrap().1.len()), // unequal channel length bug here
                            move |acc, (_, samples)| {
                                samples
                                    .iter()
                                    .enumerate()
                                    .map(|(i, s)| {
                                        acc.get(i).unwrap_or(&0f32) + (*s / (track.len() as f32))
                                    })
                                    .collect()
                            },
                        ),
                    )
                })
                .collect::<HashMap<String, Samples>>(),
        ))
    }
}
