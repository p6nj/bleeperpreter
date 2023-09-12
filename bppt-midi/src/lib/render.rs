use crate::structs::{Channel, Song};
use apres::MIDI;

impl Song {
    fn setup(&self, mid: &mut MIDI) {
        if self.channels.is_empty() {
            return;
        }
        let first = self.channels.first().unwrap();
        mid.insert_event(0, 0, apres::MIDIEvent::TrackName(first.0.clone()));
        let mut append = |e| mid.push_event(0, 0, e);
        if let Some(sig) = &self.global.signature {
            append(apres::MIDIEvent::TimeSignature(
                sig.numerator,
                sig.denominator / 2,
                24,
                8,
            ));
        }
        if let Some(key) = &self.global.key {
            append(apres::MIDIEvent::KeySignature(key.clone()));
        }
        append(apres::MIDIEvent::SetTempo(self.global.bpm.into()));
        append(apres::MIDIEvent::AllControllersOff(0));
        append(apres::MIDIEvent::ProgramChange(
            first.1.bank,
            first.1.instrument,
        ));
        if let Some(vol) = first.1.volume {
            append(apres::MIDIEvent::Volume(0, vol));
        }
        if let Some(pan) = first.1.pan {
            append(apres::MIDIEvent::Pan(0, pan));
        }
        append(apres::MIDIEvent::EffectsLevel(0, 0));
        append(apres::MIDIEvent::ChorusLevel(0, 0));
    }

    pub fn render(&self) -> MIDI {
        let mut mid = MIDI::new();
        self.setup(&mut mid);
        // render every channel by invoking its setup method after inserting a track name
        mid
    }
}

impl Channel {
    fn setup(&self, mid: &mut MIDI) {
        let mut append = |e| mid.push_event(0, 0, e);
        append(apres::MIDIEvent::AllControllersOff(0));
        append(apres::MIDIEvent::ProgramChange(self.bank, self.instrument));
        if let Some(vol) = self.volume {
            append(apres::MIDIEvent::Volume(0, vol));
        }
        if let Some(pan) = self.pan {
            append(apres::MIDIEvent::Pan(0, pan));
        }
        append(apres::MIDIEvent::EffectsLevel(0, 0));
        append(apres::MIDIEvent::ChorusLevel(0, 0));
    }
}
