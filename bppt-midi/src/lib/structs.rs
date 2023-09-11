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

#[derive(new, Debug)]
pub struct Signature {
    pub numerator: u8,
    pub denominator: u8,
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split('/');
        let numerator = split.next().unwrap().parse::<u8>().unwrap();
        let denominator = split.next().unwrap().parse::<u8>().unwrap();
        Ok(Signature::new(numerator, denominator))
    }
}

#[derive(Deserialize, new, Debug)]
pub struct Global {
    pub bpm: u16,
    pub soundfont: String,
    pub signature: Option<Signature>,
    pub key: Option<String>,
}

#[derive(Deserialize, new, Debug)]
pub struct Channel {
    pub bank: u8,
    pub instrument: u8,
    #[serde(flatten)]
    pub notes: Notes,
    pub volume: Option<u8>,
    pub pan: Option<u8>,
}
