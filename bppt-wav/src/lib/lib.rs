#![feature(iterator_try_reduce)]

mod mixing;
mod playing;
mod saving;
mod structure;

pub use playing::play;
pub use saving::export;
pub use structure::{Channel, Signal, Track};
