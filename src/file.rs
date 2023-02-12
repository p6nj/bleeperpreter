use anyhow::{Context, Ok, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::read_to_string;

pub fn read(filename: String) -> Result<String> {
    Ok(read_to_string(&filename).with_context(|| format!("Error reading file {:?}", filename))?)
}

/// Write samples to a wave file
pub fn write(filename: String, samples: Vec<i32>, channels: u16) -> Result<()> {
    use crate::audio::SAMPLE_RATE;
    let mut writer = WavWriter::create(
        &filename,
        WavSpec {
            channels: channels, // suggestion: one instrument channel for each audio channel (no additional audio processing)
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Int,
        },
    )
    .with_context(|| format!("Error creating output file {:?}", filename))?;
    for sample in samples {
        writer.write_sample(sample).with_context(|| {
            format!(
                "Error writing sample {:?} to output file {:?}",
                sample, filename
            )
        })?;
    }
    // finalize returns a wrapped `()`, reusing it for return value
    Ok(writer
        .finalize()
        .with_context(|| format!("Error updating wav header of {:?}", filename))?)
}
