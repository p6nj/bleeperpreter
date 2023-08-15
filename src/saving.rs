use crate::{backbone::SAMPLE_RATE, processing::MixedRoot};
use anyhow::{Ok, Result};
use hound::{SampleFormat, WavSpec};
use std::{fs::create_dir, path::Path};

pub(crate) fn save<P: AsRef<Path>>(mix: &MixedRoot, dir: P) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    mix.iter()
        .try_for_each(|(album, (artist, album_data))| -> Result<()> {
            let album_dir = dir.as_ref().join(format!("{} - {}", artist, album));
            mkdir(&album_dir)?;
            album_data
                .iter()
                .try_for_each(|(track, track_data)| -> Result<()> {
                    let filename = album_dir.join(track).with_extension("wav");
                    let mut writer = hound::WavWriter::create(filename, spec)?;
                    track_data
                        .iter()
                        .map(|sample| (sample * (i16::MAX as f32)) as i16)
                        .try_for_each(|sample| writer.write_sample(sample))?;
                    Ok(())
                })?;
            Ok(())
        })?;
    Ok(())
}

fn mkdir<P: AsRef<Path>>(path: P) -> Result<()> {
    if !path.as_ref().exists() {
        create_dir(path)?;
    };
    Ok(())
}
