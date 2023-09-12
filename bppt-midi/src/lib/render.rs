use crate::structs::{Channel, Song};
use anyhow::{Context, Result};
use apres::MIDI;

impl Song {
    fn setup(&self) -> Result<MIDI> {
        let mut mid = MIDI::new();
        let first = self.channels.first().context("No channels to parse")?;
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
        Ok(mid)
    }
    pub fn render(&self) -> Result<()> {
        let mut mid = self.setup();
        Ok(())
    }
}

impl Channel {
    fn setup(&self) -> MIDI {
        let mut midi = MIDI::new();
        midi
    }
}
