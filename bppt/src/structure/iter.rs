use super::*;

impl Notes {
    pub fn flat_iter(&self) -> FlattenedNoteIterator {
        FlattenedNoteIterator(self.score.iter().rev().cloned().collect::<Vec<Atom>>())
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
                            } else if let Atom::Rest(tup) = atom {
                                return Atom::Rest(
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
