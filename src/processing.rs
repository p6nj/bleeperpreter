use std::collections::HashMap;

use crate::backbone::{self, Samples};
type Channel = HashMap<String, Samples>;
type Track = HashMap<String, Channel>;
type Album = HashMap<String, Track>;
impl backbone::Track {
    pub fn process(&self) -> Track {
        Track::new()
    }
}
impl backbone::Album {
    pub fn process(&self) -> Album {
        Album::new()
    }
}
impl backbone::Channel {
    pub fn process(&self) -> Channel {
        Channel::new()
    }
}
