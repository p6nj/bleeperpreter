use apres::MIDI;
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use std::{sync::Arc, thread::sleep, time::Duration};
use tinyaudio::{run_output_device, OutputDeviceParameters};

mod bytes_utils;
use bytes_utils::as_bytes;

pub fn play(midi: &MIDI, font: SoundFont, bpm: u16) {
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut samples: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let sound_font = Arc::new(font);

    // Load the MIDI file.
    let midi_file = Arc::new(MidiFile::new(&mut as_bytes(&midi).as_slice()).unwrap());

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
    sleep(Duration::from_secs_f64(
        ((0..midi.count_tracks())
            .map(|n| midi.get_track_length(n))
            .max()
            .unwrap() as f64
            - 1f64)
            / ((bpm as f64) * 8f64),
    ));
}
