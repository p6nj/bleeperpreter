use super::*;
use anyhow::{Context, Result};

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
        let numerator = 240 * 48000 + self.remainder;
        let denominator = NonZeroUsize::from(self.bpm)
            .checked_mul(self.length.into())
            .context("Overflow: bpm * length")?
            .checked_mul(self.tup)
            .context("Overflow: (bpm * length) * tuple")?;
        self.remainder = numerator.rem_euclid(denominator.into());
        Ok(numerator / denominator)
    }
}

impl Decoder {
    pub(super) fn decode(
        &mut self,
        channel: &structure::Channel,
        gen: impl Fn(NonZeroUsize, u8, u8, u8) -> Vec<f32>,
    ) -> Result<Vec<f32>> {
        Ok(channel
            .notes
            .score
            .iter()
            .map(|atom| {
                match atom {
                    Atom::Octave(o) => self.octave = u8::from(*o) - 1,
                    Atom::Length(l) => {
                        self.length = *l;
                    }
                    Atom::Volume(v) => self.volume = *v,
                    Atom::Note(n, _) => {
                        let length = self.real_length()?;
                        if length != 0 {
                            return Ok(Some(gen(
                                NonZeroUsize::new(length).unwrap(),
                                *n,
                                self.octave,
                                self.volume,
                            )));
                        }
                    }
                    Atom::Rest => return Ok(Some(vec![0f32; self.real_length()?])),
                    Atom::OctaveIncr => self.octave += 1,
                    Atom::OctaveDecr => self.octave -= 1,
                    Atom::VolumeIncr => self.volume += 1,
                    Atom::VolumeDecr => self.volume -= 1,
                    Atom::LengthIncr => {
                        self.length
                            .checked_mul(NonZeroU8::new(2).unwrap())
                            .context("Length overflow")?;
                    }
                    Atom::LengthDecr => {
                        self.length =
                            NonZeroU8::new(u8::from(self.length) / NonZeroU8::new(2).unwrap())
                                .context("Length underflow")?;
                    }
                    Atom::Loop(_) => todo!(),
                };
                Ok(None)
            })
            .collect::<Result<Vec<Option<Vec<f32>>>>>()?
            .iter()
            .flatten()
            .flatten()
            .cloned()
            .collect())
    }
}
