use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;

use apres::MIDIEvent::{NoteOff, NoteOn};
use apres::MIDI;
use basic_toml::{from_str, to_string};
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

    // println!(
    //     "{:?}",
    //     from_str::<Song>(&read_to_string("bppt-midi/toml/poc.toml").unwrap()).unwrap()
    // );

    let midi_file = MIDI::from_path("C at 60 BPM.mid").unwrap();
    dbg!(midi_file);
}

#[derive(Deserialize, new, Debug)]
struct Song {
    global: Global,
    #[serde(flatten)]
    channels: HashMap<String, Channel>,
}

#[derive(Deserialize, new, Debug)]
struct Global {
    bpm: u16,
    soundfont: String,
}

#[derive(Deserialize, new)]
struct Channel {
    bank: u8,
    instrument: u8,
    #[serde(flatten)]
    notes: Notes,
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
