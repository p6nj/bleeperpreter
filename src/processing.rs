use std::collections::HashMap;

use crate::backbone::{self, Samples};
type Track = HashMap<String, Samples>;
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
    pub fn process(&self) -> Samples {
        Samples::new()
    }
}
impl backbone::Root {
    pub fn process(&self) -> Root {
        Root::new()
    }
}
