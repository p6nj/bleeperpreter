use derive_new::new;
use std::fmt::Debug;
use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};

mod de;
mod default;
mod iter;

/// N container. Stores mask atoms from the score and the length of the set used to calculate the notes frequencies.
/// To iterate through flattened mask atoms (without container atoms), use [`flat_iter`](fn@Notes::flat_iter).
#[derive(new, PartialEq, Debug, Clone)]
pub struct Notes {
    /// Length of the note set used to calculate note frequencies.
    pub set: u8,
    pub(crate) score: Vec<Atom>,
}

/// Mask atoms are musical bricks from the score that either indicate :
/// - a sound (or silence)
/// - a command (to change parametters)
/// - a wrapper (an element applying rules on the atoms it contains)
/// To iterate through flattened mask atoms (without container atoms), use [`Notes::flat_iter(&self)`].
#[derive(PartialEq, Debug, Clone)]
pub enum Atom {
    /// Set octave
    O(NonZeroU8),
    /// Set length
    L(NonZeroU8),
    /// Set volume
    V(u8),
    /// Play a note from the set using the set index and the tuple level (a number to divide the length by if the note is in a tuple, 1 by default)
    N(u8, NonZeroUsize),
    /// Play a rest
    Rest(NonZeroUsize),
    /// Increase the octave
    OIncr,
    /// Decrease the octave
    ODecr,
    /// Increase the length
    LIncr,
    /// Decrease the length
    LDecr,
    /// Increase the volume
    VIncr,
    /// Decrease the volume
    VDecr,
    /// Extend the previous note (implementation may vary)
    More,
    /// Loop the contained atom sequence n times
    Loop(NonZeroU16, Vec<Atom>),
    /// Tuplet : alter the contained atoms so that the total of their length equals the length of a single note
    Tuplet(Vec<Atom>),
}
