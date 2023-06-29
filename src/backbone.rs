use std::{collections::HashMap, ops::Index};

use anyhow::{Context, Error};
use json::JsonValue;
use meval::{tokenizer::Operation, Expr};
use nom::{character::complete::one_of, combinator::map_res};

pub struct Album {
    name: String,
    artist: String,
    tracks: Vec<Track>,
}

struct Track {
    name: String,
    bpm: u16,
    instruments: HashMap<String, Instrument>,
    effects: HashMap<String, Effect>,
    channels: Vec<Channel>,
}

enum Instrument {
    Sample {
        data: Vec<f32>,
        r#loops: bool,
        resets: bool,
    },
    Expression {
        expr: Expr,
        resets: bool,
    },
    Mix {
        first: Box<Instrument>,
        second: Box<Instrument>,
        operator: Operation,
    },
}

// TODO: complete effect structure
struct Effect {
    base: String,
    mask: Mask,
}

struct Channel {
    name: String,
    instrument: Instrument,
    effects: Vec<Effect>,
    tuning: u16,
    mask: Mask,
}

enum MaskAtoms {
    Octave(u8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

struct Mask(Vec<MaskAtoms>);

impl From<JsonValue> for Channel {
    fn from(value: JsonValue) -> Self {
        todo!()
    }
}

/// From the string mask and a string of allowed notes
impl TryFrom<(String, String)> for Mask {
    type Error = Error;
    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        Ok(Self(vec![MaskAtoms::Note(map_res(
            one_of(value.1),
            move |note| -> u8 { value.1.find(note).unwrap().try_into() },
        )(value.0))]))
    }
}
