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
            .flat_iter()
            .map(|atom| {
                match atom {
                    Atom::O(o) => self.octave = u8::from(o) - 1,
                    Atom::L(l) => {
                        self.length = l;
                    }
                    Atom::V(v) => self.volume = v,
                    Atom::N(n, tup) => {
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
                    Atom::Rest(tup) => {
                        self.tup = tup;
                        return Ok(Some(vec![0f32; self.real_length()?]));
                    }
                    Atom::OIncr => self.octave += 1,
                    Atom::ODecr => self.octave -= 1,
                    Atom::VIncr => self.volume += 1,
                    Atom::VDecr => self.volume -= 1,
                    Atom::LIncr => {
                        self.length = self
                            .length
                            .checked_mul(NonZeroU8::new(2).unwrap())
                            .with_context(|| {
                                format!("L overflow, already at length {}", self.length)
                            })?;
                    }
                    Atom::LDecr => {
                        self.length =
                            NonZeroU8::new(u8::from(self.length) / NonZeroU8::new(2).unwrap())
                                .with_context(|| {
                                    format!("L underflow, already at length {}", self.length)
                                })?;
                    }
                    Atom::Loop(_, _) | Atom::Tuplet(_) => unreachable!(
                        "Loops and tuplets should be flattened by the FlattenedNoteIterator"
                    ),
                    Atom::More => {
                        unimplemented!("Sorry, '+' operator is unimplemented at the moment")
                    }
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
