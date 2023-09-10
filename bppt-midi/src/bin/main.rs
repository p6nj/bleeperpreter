use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::{read_to_string, File};
use std::sync::Arc;

use apres::MIDIEvent::{self, NoteOff, NoteOn};
use apres::{MIDIBytes, MIDI};
use basic_toml::{from_str, to_string};
use bppt::Notes;
use derive_new::new;
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use tinyaudio::{run_output_device, OutputDeviceParameters};

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

    let midi_file = MIDI::from_path("bppt-midi/midi/C at 60 BPM oboe.mid").unwrap();

    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut samples: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut sf2 = File::open("bppt-midi/soundfont/soundfont.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let midi_file = Arc::new(MidiFile::new(&mut as_bytes(&midi_file).as_slice()).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            let mut empty = vec![0f32; samples.len()];
            sequencer.render(&mut samples[..], &mut empty);
            for (i, value) in samples.iter().enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();

    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(4));
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

fn as_bytes(midi: &MIDI) -> Vec<u8> {
    // First 8  bytes will always be the same
    let mut output: Vec<u8> = vec!['M' as u8, 'T' as u8, 'h' as u8, 'd' as u8, 0, 0, 0, 6];

    let format: u16 = midi.get_format();
    output.push((format / 256) as u8);
    output.push((format % 256) as u8);

    let track_count: u16 = midi.count_tracks() as u16;
    output.push((track_count / 256) as u8);
    output.push((track_count % 256) as u8);

    let ppqn: u16 = midi.get_ppqn();
    output.push((ppqn / 256) as u8);
    output.push((ppqn % 256) as u8);

    // Tracks (MTrk)
    let mut track_event_bytes: Vec<u8>;
    let mut track_byte_length: u32;
    let tracks: Vec<Vec<(usize, u64)>> = midi.get_tracks();

    for ticks in tracks.iter() {
        output.push('M' as u8);
        output.push('T' as u8);
        output.push('r' as u8);
        output.push('k' as u8);

        track_event_bytes = Vec::new();
        for (tick_delay, eid) in ticks.iter() {
            match midi.get_event(*eid) {
                Some(working_event) => {
                    track_event_bytes.extend(to_variable_length_bytes(*tick_delay).iter().copied());
                    track_event_bytes.extend(working_event.as_bytes());
                }
                None => {}
            }
        }

        // Automatically handle EndOfTrackEvent Here instead of requiring it be in the MIDITrack Object
        track_event_bytes.push(0);
        track_event_bytes.extend(MIDIEvent::EndOfTrack.as_bytes().iter().copied());

        // track length in bytes
        track_byte_length = track_event_bytes.len() as u32;
        output.push((track_byte_length / 256_u32.pow(3)) as u8);
        output.push(((track_byte_length / 256_u32.pow(2)) % 256) as u8);
        output.push(((track_byte_length / 256_u32.pow(1)) % 256) as u8);
        output.push((track_byte_length % 256) as u8);

        output.extend(track_event_bytes.iter().copied());
    }

    output
}

fn to_variable_length_bytes(number: usize) -> Vec<u8> {
    let mut output = Vec::new();
    let mut first_pass = true;
    let mut working_number = number;
    let mut tmp;
    while working_number > 0 || first_pass {
        tmp = working_number & 0x7F;
        working_number >>= 7;

        if !first_pass {
            tmp |= 0x80;
        }

        output.push(tmp as u8);
        first_pass = false;
    }
    output.reverse();

    output
}
