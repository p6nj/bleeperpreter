use anyhow::{Context, Result};
use std::fs::{read_to_string, File};
use std::io::prelude::*;

pub fn read_file(filename: String) -> Result<String> {
    let data: String =
        read_to_string(&filename).with_context(|| format!("Error reading file {:?}", filename))?;
    let mut parsed_data: String = data
        .parse()
        .with_context(|| format!("Error parsing file {:?}", filename))?;
    parsed_data.pop(); // the last line is always empty
    Ok(parsed_data)
}

pub fn gen_output_file(filename: String) -> Result<(File, u64)> {
    write_header(File::create(filename).context("Cannot create output file.")?)
}

fn write_header(mut output_file: File) -> Result<(File, u64)> {
    const BITDEPTH: u16 = 16;
    const SAMPLERATE: u32 = 44_100;
    const CHANNELS: u16 = 1;
    const BLOCKALIGN: u16 = BITDEPTH / 2;
    const BYTERATE: u32 = SAMPLERATE * BITDEPTH as u32 / 8;
    const FORMAT: u16 = 1; // WAVE_FORMAT_PCM
    const CHUNKSIZE: u32 = 16;
    const DURATION: u8 = 2;
    const FREQUENCY: f64 = 442.;
    //- RIFF
    output_file.write_all(&[0x52, 0x49, 0x46, 0x46])?;
    // - place holder
    let pos_cksize = output_file.stream_position()?;
    output_file.write_all("----".as_bytes())?;
    output_file.write_all("WAVE".as_bytes())?;
    output_file.write_all("fmt ".as_bytes())?;

    // Format
    output_file.write_all(&CHUNKSIZE.to_le_bytes())?;
    output_file.write_all(&FORMAT.to_le_bytes())?;
    output_file.write_all(&CHANNELS.to_le_bytes())?;
    output_file.write_all(&SAMPLERATE.to_le_bytes())?;
    output_file.write_all(&BYTERATE.to_le_bytes())?;
    output_file.write_all(&BLOCKALIGN.to_le_bytes())?;
    output_file.write_all(&BITDEPTH.to_le_bytes())?;

    Ok((output_file, pos_cksize))
}
