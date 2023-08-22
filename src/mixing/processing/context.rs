use super::*;

impl Context {
    pub(super) fn new(bpm: u16) -> Self {
        Context {
            bpm,
            octave: 4,
            length: 4,
            volume: 100,
            remainder: 0,
        }
    }
    fn real_length(&mut self) -> usize {
        let numerator = 240 * 48000 + self.remainder;
        let denominator = (self.bpm as usize) * (self.length as usize);
        self.remainder = numerator.rem_euclid(denominator);
        numerator / denominator
    }
}

impl Context {
    pub(super) fn parse(
        &mut self,
        channel: &structure::Channel,
        gen: impl Fn(usize, u8, u8, u8) -> Vec<f32>,
    ) -> Vec<f32> {
        channel
            .notes
            .score
            .iter()
            .map(|atom| {
                match atom {
                    MaskAtom::Octave(o) => self.octave = u8::from(*o) - 1,
                    MaskAtom::Length(l) => {
                        self.length = *l;
                    }
                    MaskAtom::Volume(v) => self.volume = *v,
                    MaskAtom::Note(n) => {
                        return Some(gen(self.real_length(), *n, self.octave, self.volume));
                    }
                    MaskAtom::Rest => return Some(vec![0f32; self.real_length()]),
                    MaskAtom::OctaveIncr => self.octave += 1,
                    MaskAtom::OctaveDecr => self.octave -= 1,
                    MaskAtom::VolumeIncr => self.volume += 1,
                    MaskAtom::VolumeDecr => self.volume -= 1,
                    MaskAtom::LengthIncr => {
                        self.length *= 2;
                    }
                    MaskAtom::LengthDecr => {
                        self.length /= 2;
                    }
                };
                None
            })
            .flatten()
            .flatten()
            .collect()
    }
}
