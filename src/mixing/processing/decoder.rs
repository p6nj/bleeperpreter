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
            .into_iter()
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
                        self.length
                            .checked_mul(NonZeroU8::new(2).unwrap())
                            .context("Length overflow")?;
                    }
                    Atom::LengthDecr => {
                        self.length =
                            NonZeroU8::new(u8::from(self.length) / NonZeroU8::new(2).unwrap())
                                .context("Length underflow")?;
                    }
                    _ => unreachable!("Loops and tuplets should be flattened by the NoteIterator"),
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

impl IntoIterator for Notes {
    type Item = Atom;
    type IntoIter = NoteIterator;

    fn into_iter(self) -> Self::IntoIter {
        NoteIterator({
            let mut v = self.score;
            v.reverse();
            v
        })
    }
}

pub(crate) struct NoteIterator(Vec<Atom>);

impl Iterator for NoteIterator {
    type Item = Atom;
    fn next(&mut self) -> Option<Self::Item> {
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
                self.0.pop()
            }
            Some(Atom::Tuplet(v)) => {
                let mut v = v
                    .iter()
                    .map(|atom| {
                        if let Atom::Note(n, tup) = atom {
                            return Atom::Note(*n, tup.saturating_add(v.len()));
                        }
                        atom.clone()
                    })
                    .cycle()
                    .take(v.len())
                    .collect::<Vec<Atom>>();
                v.reverse();
                self.0.append(&mut v);
                self.0.pop()
            }
            other => other,
        }
    }
}
