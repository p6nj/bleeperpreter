use super::Notes;
use meval::Expr;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::str::FromStr;

mod atoms;
pub(crate) use self::atoms::Atom;
mod notes;
mod signal;
pub use self::signal::Signal;
