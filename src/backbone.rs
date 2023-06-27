use std::collections::HashMap;

use meval::{tokenizer::Operation, Expr};

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
    mask: Vec<MaskAtoms>,
}

struct Channel {
    name: String,
    instrument: Instrument,
    effect: Vec<Effect>,
    tuning: u16,
    mask: Vec<MaskAtoms>,
}

enum MaskAtoms {
    Octave(u8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}
