use super::*;

impl Context {
    pub(super) fn new(bpm: u16, channel: &structure::Channel) -> Result<Self> {
        Ok(Context {
            bpm,
            octave: 4,
            length: 4,
            volume: 100,
            tuplet: 1,
            counter: vec![].into(),
            remainder: 0,
            generator: Box::new(channel.generator()?),
            score: channel.notes.score.iter().rev().cloned().collect(),
        })
    }
    fn real_length(&mut self) -> usize {
        let numerator = 240 * 48000 + self.remainder;
        let denominator = (self.bpm as usize) * (self.length as usize);
        self.remainder = numerator.rem_euclid(denominator);
        numerator / denominator
    }
    fn gen(&mut self, n: u8) -> Vec<f32> {
        let real = self.real_length();
        (self.generator)(real, n, self.octave, self.volume)
    }
    fn step(&mut self) {
        if let Some((tuplet, countdown)) = self.counter.pop_front() {
            if countdown > 0 {
                self.counter.insert(0, (tuplet, countdown - 1));
            } else {
                self.tuplet /= tuplet
            }
        }
    }
}

impl Iterator for Context {
    type Item = Vec<f32>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(atom) = self.score.pop() {
            self.step();
            match atom {
                MaskAtom::Octave(o) => self.octave = u8::from(o) - 1,
                MaskAtom::Length(l) => {
                    self.length = l;
                }
                MaskAtom::Volume(v) => self.volume = v,
                MaskAtom::Note(n) => {
                    return Some(self.gen(n));
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
                MaskAtom::Loop(v) => self
                    .score
                    .append(&mut v.iter().rev().cloned().cycle().take(v.len() * 2).collect()),
                MaskAtom::Tuplet(v) => {
                    self.tuplet *= v.len();
                    self.score.append(&mut v.iter().rev().cloned().collect());
                }
            }
        }
        None
    }
}
