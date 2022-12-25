use anyhow::{Context, Ok, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::read_to_string;

/// Read a file and return its content in a String
pub fn read(filename: String) -> Result<String> {
    let data: String =
        read_to_string(&filename).with_context(|| format!("Error reading file {:?}", filename))?;
    let mut parsed_data: String = data
        .parse()
        .with_context(|| format!("Error parsing file {:?}", filename))?;
    parsed_data.pop(); // the last line is always empty
    Ok(parsed_data)
}

/// Write samples to a wave file
pub fn write(filename: String, samples: Vec<f32>) -> Result<()> {
    use crate::audio::SAMPLE_RATE;
    let mut writer = WavWriter::create(
        filename,
        WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
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
    // finalize returns a wrapped `()`
    Ok(writer
        .finalize()
        .with_context(|| format!("Error updating wav header of {:?}", filename))?)
}
