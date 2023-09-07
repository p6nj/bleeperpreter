use std::str::FromStr;

use meval::Expr;
use serde_json::from_str;

use super::*;

impl Default for Track {
    fn default() -> Self {
        Self {
            bpm: NonZeroU16::new(120).unwrap(),
            channels: vec![Channel::default()],
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            signal: Signal::default(),
            notes: Notes::default(),
            tuning: 442.0,
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self(Expr::from_str("sin(2*pi*f*t)").unwrap())
    }
}
