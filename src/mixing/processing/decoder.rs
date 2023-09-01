use super::*;
use crate::structure::Notes;
use anyhow::Result;

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

impl Notes {
    pub fn flat_iter(self) -> FlattenedNoteIterator {
        FlattenedNoteIterator({
            let mut v = self.score;
            v.reverse();
            v
        })
    }
}

pub struct FlattenedNoteIterator(Vec<Atom>);

impl Iterator for FlattenedNoteIterator {
    type Item = Atom;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.0.pop();
            match next {
                Some(Atom::Loop(repeat, v)) => {
                    let mut v = v
                        .iter()
                        .cloned()
                        .cycle()
                        .take(v.len() * usize::from(NonZeroUsize::from(repeat)))
                        .collect::<Vec<Atom>>();
                    v.reverse();
                    self.0.append(&mut v);
                }
                Some(Atom::Tuplet(v)) => {
                    let mut v = Notes::new(String::new(), v)
                        .flat_iter()
                        .collect::<Vec<Atom>>();
                    let length = v.len();
                    v = v
                        .iter()
                        .map(|atom| {
                            if let Atom::Note(n, tup) = atom {
                                return Atom::Note(
                                    *n,
                                    tup.saturating_mul(NonZeroUsize::new(length).unwrap()),
                                );
                            }
                            atom.clone()
                        })
                        .cycle()
                        .take(length)
                        .collect::<Vec<Atom>>();
                    v.reverse();
                    self.0.append(&mut v);
                }
                other => break other,
            }
        }
    }
}
