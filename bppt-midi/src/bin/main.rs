use std::collections::HashMap;

use apres::MIDIEvent::{NoteOff, NoteOn};
use apres::MIDI;
use basic_toml::to_string;
use bppt::Notes;
use derive_new::new;
use rustysynth::SoundFont;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

fn main() {
    // // Create an empty MIDI file.
    // let mut midi = MIDI::new();

    // // Using channel 0, press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
    // midi.insert_event(0, 0, NoteOn(0, 64, 100));

    // // Still on channel 0, release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
    // midi.push_event(0, 120, NoteOff(0, 64, 100));

    // // Save it to a file
    // midi.save("beep.mid");
    let song = Song::new(
        Global::new(60, "smth.sf2".to_string()),
        HashMap::from([
            (
                "first".to_string(),
                Channel::new(0, 0, Notes::new("set".to_string(), vec![])),
            ),
            (
                "next".to_string(),
                Channel::new(0, 0, Notes::new("set".to_string(), vec![])),
            ),
        ]),
    );
    println!("{}", to_string(&song).unwrap());
}

#[derive(Deserialize, Serialize, new)]
struct Song {
    global: Global,
    #[serde(flatten)]
    channels: HashMap<String, Channel>,
}

#[derive(Deserialize, Serialize, new)]
struct Global {
    bpm: u16,
    soundfont: String,
}

#[derive(Deserialize, new)]
struct Channel {
    bank: u8,
    instrument: u8,
    notes: Notes,
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("bank", &self.bank)?;
        map.serialize_entry("instrument", &self.instrument)?;
        map.serialize_entry("notes", "notes go here")?;
        map.end()
    }
}
