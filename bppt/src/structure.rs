use derive_new::new;
use std::fmt::Debug;
use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};

mod de;
mod default;
mod iter;

#[derive(new, PartialEq, Debug, Clone)]
pub struct Notes {
    pub set: u8,
    pub(crate) score: Vec<Atom>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Atom {
    Octave(NonZeroU8),
    Length(NonZeroU8),
    V(u8),
    Note(u8, NonZeroUsize),
    Rest(NonZeroUsize),
    OctaveIncr,
    OctaveDecr,
    LengthIncr,
    LengthDecr,
    VIncr,
    VDecr,
    More,
    Loop(NonZeroU16, Vec<Atom>),
    Tuplet(Vec<Atom>),
}
