use super::*;
use std::collections::VecDeque;
mod context;

impl structure::Track {
    fn process(&mut self) -> Result<Track> {
        self.channels
            .par_iter_mut()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process(self.bpm)?))
            })
            .collect::<Result<Track>>()
    }
}

impl structure::Album {
    pub(super) fn process(&mut self) -> Result<Album> {
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
}

impl structure::Channel {
    fn process(&mut self, bpm: u16) -> Result<Samples> {
        Ok(Context::new(bpm, self)?.flatten().collect())
    }
}

struct Context {
    bpm: u16,
    octave: u8,
    length: u8,
    volume: u8,
    tuplet: usize,
    counter: VecDeque<(usize, usize)>,
    remainder: usize,
    score: Vec<MaskAtom>,
    generator: Box<dyn Fn(usize, u8, u8, u8) -> Vec<f32>>,
}
