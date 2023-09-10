use apres::MIDI;
use basic_toml;
use bppt_midi::misc::bpm_to_tempo;
use bppt_midi::playback::play;
use bppt_midi::structs::Song;
use rustysynth::SoundFont;
use std::fs::{read_to_string, File};
use std::path::Path;

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
    let song: Song = basic_toml::from_str(
        read_to_string(Path::new("bppt-midi/toml/poc.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    play(
        &midi_file,
        SoundFont::new(&mut File::open(song.global.soundfont.as_str()).unwrap()).unwrap(),
        song.global.bpm,
    );
}
