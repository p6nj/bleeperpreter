use super::Notes;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use meval::Expr;
use std::str::FromStr;

mod atoms;
pub(crate) use self::atoms::Atom;
mod notes;
mod signal;
pub(crate) use self::signal::Signal;