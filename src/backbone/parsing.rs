use crate::backbone::Mask;

pub(crate) use self::logos::MaskAtom;
use super::{Channel, Notes};
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize,
};
mod logos;
mod parsing_errors;
#[cfg(test)]
mod tests;
