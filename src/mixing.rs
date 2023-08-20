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
                    (name.clone(), {
                        let mut sorted = track
                            .iter()
                            .map(move |(_, s)| s)
                            .cloned()
                            .collect::<Vec<Vec<f32>>>();
                        sorted.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
                        sorted.iter().cloned().fold(vec![], move |acc, v| {
                            v.iter()
                                .zip(acc.iter().chain([0f32].iter().cycle()))
                                .map(move |(s, acc)| *s / (track.len() as f32) + acc)
                                .collect()
                        })
                    })
                })
                .collect::<HashMap<String, Samples>>(),
        ))
    }
}
