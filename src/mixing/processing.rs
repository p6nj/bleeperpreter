use super::*;
use anyhow::Context;
use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};
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
        Decoder::new(bpm).decode(self, self.generator()?)
    }
}

struct Decoder {
    bpm: NonZeroU16,
    octave: u8,
    length: NonZeroU8,
    volume: u8,
    remainder: u8,
    tup: NonZeroUsize,
}

impl Decoder {
    pub(super) fn new(bpm: NonZeroU16) -> Self {
        Decoder {
            bpm,
            octave: 4,
            length: NonZeroU8::new(4).unwrap(),
            volume: 100,
            remainder: 0,
            tup: NonZeroUsize::new(1).unwrap(),
        }
    }
    fn real_length(&mut self) -> Result<usize> {
        let numerator = 48000 * 4 * 60;
        let denominator = usize::from(NonZeroUsize::from(self.bpm))
            * usize::from(NonZeroUsize::from(self.length))
            * usize::from(self.tup);
        self.remainder = (numerator % denominator)
            .try_into()
            .context("REMAINDER OVERFLOW BUG")?;
        Ok(numerator / denominator)
    }
}
