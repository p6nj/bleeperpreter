use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};

use crate::{backbone::SAMPLE_RATE, processing::MixedRoot, tags};
use anyhow::{Context, Ok, Result};
use dirs::home_dir;
use hound::{SampleFormat, WavSpec};

pub(crate) fn save(mix: &MixedRoot) -> Result<()> {
    let dir = setup()?;
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    mix.iter()
        .try_for_each(|(album, (artist, album_data))| -> Result<()> {
            let album_dir = dir.join(format!("{} - {}", artist, album));
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
                    // let filename = filename
                    //     .as_os_str()
                    //     .to_str()
                    //     .context("filename has invalid utf-8")?;
                    // tags::apply(filename, album, artist)
                    //     .with_context(|| format!("can't tag {}", filename))?;
                    Ok(())
                })?;
            Ok(())
        })?;
    Ok(())
}

fn setup() -> Result<PathBuf> {
    let dir = home_dir().context("can't get home dir")?.join("Documents");
    mkdir(&dir).context("can't create Documents directory")?;
    Ok(dir)
}

fn mkdir(path: &Path) -> Result<()> {
    if !path.exists() {
        create_dir(path)?;
    };
    Ok(())
}