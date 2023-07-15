use std::collections::HashMap;

use crate::backbone::{self, Samples};
type Channel = HashMap<String, Samples>;
type Track = HashMap<String, Channel>;
type Album = HashMap<String, Track>;
type Root = HashMap<String, Album>;
impl backbone::Track {
    pub fn process(&self) -> Track {
        self.channels
            .iter()
            .map(|(name, channel)| (name.clone(), channel.process()))
            .collect()
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
impl backbone::Root {
    pub fn process(&self) -> Root {
        Root::new()
    }
}
