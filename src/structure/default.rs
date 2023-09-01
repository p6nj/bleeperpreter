use std::str::FromStr;

use meval::Expr;
use serde_json::from_str;

use super::*;

impl Default for Root {
    fn default() -> Self {
        Self(HashMap::from([("Default".to_string(), Album::default())]))
    }
}

impl Default for Album {
    fn default() -> Self {
        Self {
            artist: "Default".to_string(),
            tracks: HashMap::from([("Default".to_string(), Track::default())]),
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            bpm: NonZeroU16::new(120).unwrap(),
            channels: HashMap::from([(String::new(), Channel::default())]),
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

impl Default for Notes {
    fn default() -> Self {
        from_str(
            r#"
        {
            "set": "aAbcCdDefFgG",
            "score": "cccd'ed`cedd''c"
        }
        "#,
        )
        .unwrap()
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self(Expr::from_str("sin(2*pi*f*t)").unwrap())
    }
}
