use super::Notes;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

mod atoms;
pub(crate) use self::atoms::Atom;
mod notes;