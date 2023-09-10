use bppt::Notes;
use derive_new::new;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

#[derive(Deserialize, new, Debug)]
pub struct Song {
    pub global: Global,
    #[serde(flatten)]
    pub channels: HashMap<String, Channel>,
}

#[derive(Deserialize, new, Debug)]
pub struct Global {
    pub bpm: u16,
    pub soundfont: String,
}

#[derive(Deserialize, new)]
pub struct Channel {
    pub bank: u8,
    pub instrument: u8,
    #[serde(flatten)]
    pub notes: Notes,
}

impl Debug for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel")
            .field("bank", &self.bank)
            .field("instrument", &self.instrument)
            .field("set", &self.notes.set)
            .field("score", &"...")
            .finish()
    }
}
