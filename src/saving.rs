use crate::{mixing::Samples, structure::SAMPLE_RATE};
use anyhow::{Ok, Result};
use hound::{SampleFormat, WavSpec};
use std::path::Path;

pub fn save<P: AsRef<Path>>(mix: Samples, path: P) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let filename = path.as_ref().with_extension("wav");
    let mut writer = hound::WavWriter::create(filename, spec)?;
    mix.iter()
        .map(|sample| (sample * (i16::MAX as f32)) as i16)
        .try_for_each(|sample| writer.write_sample(sample))?;
    Ok(())
}
