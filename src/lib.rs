#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![feature(iterator_try_reduce)]

//! <img src="https://raw.githubusercontent.com/p6nj/bleeperpreter/main/icon/iconx4.png?sanitize=true" alt="bppt logo" align="right">
//!
//! # Bleeperpreter
//!
//! A modular MML interpreter with real Math inside.
//!
//! **[Binary Releases](https://github.com/p6nj/bleeperpreter/releases) -**
//! **[Library Documentation](https://docs.rs/bpptlib/) -**
//! **[Cargo](https://crates.io/crates/bppt) -**
//! **[Repository](https://github.com/p6nj/bleeperpreter)**

mod mixing;
mod playing;
mod saving;
mod structure;

pub use playing::play;
pub use saving::save;
pub use structure::{Root, Signal};
