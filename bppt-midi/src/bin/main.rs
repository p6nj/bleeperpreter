use apres::MIDIEvent::{NoteOff, NoteOn};
use apres::MIDI;

fn main() {
    // Create an empty MIDI file.
    let mut midi = MIDI::new();

    // Using channel 0, press midi note 64 (Middle E) on the first track (0) at the first position (0 ticks)
    midi.insert_event(0, 0, NoteOn(0, 64, 100));

    // Still on channel 0, release midi note 64 (Middle E) on the first track (0) one beat later (120 ticks)
    midi.push_event(0, 120, NoteOff(0, 64, 100));

    // Save it to a file
    midi.save("beep.mid");
}
