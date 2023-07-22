use std::collections::HashMap;

use crate::backbone;
type Track = HashMap<String, Samples>;
type Album = HashMap<String, Track>;
type Root = HashMap<String, Album>;
type Samples = Vec<f32>;
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
        match &self.instrument {
            backbone::Instrument::Sample {
                wav,
                r#loops,
                resets,
            } => todo!(),
            backbone::Instrument::Expression { expr, resets } => todo!(),
        }
    }
}
impl backbone::Root {
    pub fn process(&self) -> Root {
        Root::new()
    }
}
