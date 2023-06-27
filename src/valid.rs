use std::collections::HashMap;

pub struct Album {
    name: String,
    artist: String,
    tracks: Vec<Track>,
}

pub struct Track {
    name: String,
    bpm: u16,
    instruments: HashMap<String, Instrument>,
    effects: HashMap<String, Effect>,
    channels: Vec<Channel>,
}

pub struct Instrument {}
pub struct Effect {}
pub struct Channel {}
