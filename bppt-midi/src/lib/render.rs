use crate::{
    misc::bpm_to_tempo,
    structs::{Channel, Song},
};
use apres::MIDI;
use std::num::{NonZeroU8, NonZeroUsize};

impl Song {
    fn setup(&self, mid: &mut MIDI) {
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
        append(apres::MIDIEvent::SetTempo(bpm_to_tempo(self.global.bpm)));
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
        if self.channels.is_empty() {
            return mid;
        }
        self.setup(&mut mid);
        // render every channel by invoking its setup method after inserting a track name
        self.channels.first().unwrap().1.render(&mut mid, 0);
        self.channels
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(i, (name, channel))| {
                mid.insert_event(i + 1, 0, apres::MIDIEvent::TrackName(name.clone()));
                channel.setup(&mut mid);
                channel.render(&mut mid, i + 1);
            });
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
    fn render(&self, mid: &mut MIDI, track: usize) {
        let mut id = None;
        let mut silence = 0;
        let mut context = Env::new();
        self.notes.flat_iter().for_each(|atom| match atom {
            bppt::Atom::Octave(o) => context.octave = u8::from(o),
            bppt::Atom::Length(l) => context.length = l,
            bppt::Atom::V(v) => context.velocity = v,
            bppt::Atom::Note(n, t) => {
                context.tup = t;
                let note = n + 12 * (context.octave + 2);
                mid.push_event(
                    track,
                    silence,
                    apres::MIDIEvent::NoteOn(0, note, context.velocity),
                );
                silence = 0;
                id = Some(mid.push_event(
                    track,
                    context.length() - 1,
                    apres::MIDIEvent::NoteOff(0, note, 0),
                ));
            }
            bppt::Atom::Rest(t) => {
                context.tup = t;
                silence += context.length();
            }
            bppt::Atom::OctaveIncr => context.octave += 1,
            bppt::Atom::OctaveDecr => context.octave -= 1,
            bppt::Atom::LengthIncr => {
                context.length = NonZeroU8::new(u8::from(context.length) + 1).unwrap()
            }
            bppt::Atom::LengthDecr => {
                context.length = NonZeroU8::new(u8::from(context.length) - 1).unwrap()
            }
            bppt::Atom::VIncr => context.velocity += 1,
            bppt::Atom::VDecr => context.velocity -= 1,
            bppt::Atom::More => {
                if let Some(id) = id {
                    let event = mid.get_event(id).unwrap();
                    mid.push_event(
                        0,
                        mid.get_event_position(id).unwrap().1 + context.length(),
                        event,
                    );
                    mid.move_event(usize::MAX, 0, id);
                }
            }
            bppt::Atom::Loop(_, _) | bppt::Atom::Tuplet(_) => {
                unreachable!("loops and tuplets shouldn't be here")
            }
        });
        mid.push_event(track, 0, apres::MIDIEvent::EndOfTrack);
    }
}
