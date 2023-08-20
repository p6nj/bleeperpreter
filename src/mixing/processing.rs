use super::*;

impl structure::Track {
    fn process(&mut self) -> Result<Track> {
        self.channels
            .par_iter_mut()
            .map(|(name, channel)| -> Result<(String, Samples)> {
                Ok((name.clone(), channel.process(&self.bpm)?))
            })
            .collect::<Result<Track>>()
    }
}

impl structure::Album {
    pub(super) fn process(&mut self) -> Result<Album> {
        Ok((
            self.artist.clone(),
            self.tracks
                .iter_mut()
                .map(|(name, track)| -> Result<(String, Track)> {
                    Ok((name.clone(), track.process()?))
                })
                .collect::<Result<HashMap<String, Track>>>()?,
        ))
    }
}

impl structure::Channel {
    fn process(&mut self, bpm: &u16) -> Result<Samples> {
        let sr = 48000usize;
        let mut octave = 4u8;
        let mut volume = 100u8;
        let mut result = vec![];

        let mut remainder = 0;
        let mut genlength = move |length: u8| -> Result<usize> {
            let numerator = 240 * sr + remainder;
            let denominator = (*bpm as usize) * (length as usize);
            remainder = numerator.rem_euclid(denominator);
            Ok(numerator / denominator)
        };
        let mut real_length = genlength(4)?;

        let gen = self.generator()?;
        self.notes.score.iter().try_for_each(|a| -> Result<()> {
            match *a {
                MaskAtom::Octave(o) => octave = u8::from(o) - 1,
                MaskAtom::Length(l) => real_length = genlength(l)?,
                MaskAtom::Volume(v) => volume = v,
                MaskAtom::Note(n) => result.append(&mut gen(real_length, n, octave, volume)),
                MaskAtom::Rest => result.append(&mut vec![0f32; real_length]),
            };
            Ok(())
        })?;
        Ok(result)
    }
}
