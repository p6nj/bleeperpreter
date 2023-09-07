use super::Decoder;
use crate::structure;
use anyhow::{Context, Result};
use bppt::Atom;
use std::num::{NonZeroU8, NonZeroUsize};

impl Decoder {
    pub(super) fn decode(
        &mut self,
        channel: &structure::Channel,
        gen: impl Fn(NonZeroUsize, u8, u8, u8) -> Vec<f32>,
    ) -> Result<Vec<f32>> {
        Ok(channel
            .notes
            .clone()
            .flat_iter()
            .map(|atom| {
                match atom {
                    Atom::Octave(o) => self.octave = u8::from(o) - 1,
                    Atom::Length(l) => {
                        self.length = l;
                    }
                    Atom::Volume(v) => self.volume = v,
                    Atom::Note(n, tup) => {
                        self.tup = tup;
                        let length = self.real_length()?;
                        if length != 0 {
                            return Ok(Some(gen(
                                NonZeroUsize::new(length).unwrap(),
                                n,
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
                        self.length = self
                            .length
                            .checked_mul(NonZeroU8::new(2).unwrap())
                            .with_context(|| {
                                format!("Length overflow, already at length {}", self.length)
                            })?;
                    }
                    Atom::LengthDecr => {
                        self.length =
                            NonZeroU8::new(u8::from(self.length) / NonZeroU8::new(2).unwrap())
                                .with_context(|| {
                                    format!("Length underflow, already at length {}", self.length)
                                })?;
                    }
                    _ => unreachable!(
                        "Loops and tuplets should be flattened by the FlattenedNoteIterator"
                    ),
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
