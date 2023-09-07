use super::*;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};
mod decoder;

impl structure::Track {
    pub(super) fn process(&mut self) -> Result<Vec<Samples>> {
        self.channels
            .par_iter_mut()
            .map(|channel| -> Result<Samples> { Ok(channel.process(self.bpm)?) })
            .collect::<Result<Vec<Samples>>>()
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
    remainder: usize,
    tup: NonZeroUsize,
}

impl Decoder {
    pub(super) fn new(bpm: NonZeroU16) -> Self {
        Decoder {
            bpm,
            octave: 3,
            length: NonZeroU8::new(4).unwrap(),
            volume: 100,
            remainder: 0,
            tup: NonZeroUsize::new(1).unwrap(),
        }
    }
    fn real_length(&mut self) -> Result<usize> {
        let numerator = 48000 * 4 * 60 + self.remainder;

        let denominator = usize::from(NonZeroUsize::from(self.bpm))
            * usize::from(NonZeroUsize::from(self.length))
            * usize::from(self.tup);

        self.remainder = numerator % denominator;

        Ok(numerator / denominator)
    }
}
