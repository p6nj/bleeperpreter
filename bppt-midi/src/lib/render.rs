use std::num::{NonZeroU8, NonZeroUsize};

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
        // after that add the notes and an end of track event
        mid
    }
}

struct Env {
    octave: u8,
    length: NonZeroU8,
    velocity: u8,
    remainder: usize,
    tup: NonZeroUsize,
}

impl Env {
    fn new() -> Self {
        Env {
            octave: 3,
            length: NonZeroU8::new(4).unwrap(),
            velocity: 100,
            remainder: 0,
            tup: NonZeroUsize::new(1).unwrap(),
        }
    }
    fn length(&mut self) -> usize {
        const NUMERATOR: usize = 1920;
        let denominator = usize::from(u8::from(self.length)) * usize::from(self.tup);
        self.remainder = NUMERATOR % denominator;
        NUMERATOR / denominator
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
    fn render(self, mid: &mut MIDI) {
        let mut append = |e, w| {
            mid.push_event(0, w, e);
        };
        let mut context = Env::new();
        self.notes.flat_iter().for_each(|atom| match atom {
            bppt::Atom::Octave(o) => context.octave = u8::from(o),
            bppt::Atom::Length(l) => context.length = l,
            bppt::Atom::Volume(v) => context.velocity = v,
            bppt::Atom::Note(n, t) => {
                context.tup = t;
                append(
                    apres::MIDIEvent::NoteOn(0, n * context.octave, context.velocity),
                    context.length(),
                )
            }
            bppt::Atom::Rest(t) => {
                context.tup = t;
                append(apres::MIDIEvent::NoteOff(0, 0, 0), context.length())
            }
            bppt::Atom::OctaveIncr => todo!(),
            bppt::Atom::OctaveDecr => todo!(),
            bppt::Atom::LengthIncr => todo!(),
            bppt::Atom::LengthDecr => todo!(),
            bppt::Atom::VolumeIncr => todo!(),
            bppt::Atom::VolumeDecr => todo!(),
            bppt::Atom::More => todo!(),
            bppt::Atom::Loop(_, _) => todo!(),
            bppt::Atom::Tuplet(_) => todo!(),
        });
    }
}
