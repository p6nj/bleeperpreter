use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};

use super::*;
mod decoder;

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
    fn process(&mut self, bpm: NonZeroU16) -> Result<Samples> {
        Ok(Decoder::new(bpm).decode(&self, self.generator()?)?)
    }
}

struct Decoder {
    bpm: NonZeroU16,
    octave: u8,
    length: NonZeroU8,
    volume: u8,
    remainder: usize,
    tup: NonZeroUsize,
}
