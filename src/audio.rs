use crate::file::gen_output_file;
use anyhow::{Context, Result};
use std::f64::consts::PI;
use std::io::prelude::*;
use std::io::SeekFrom;

const BITDEPTH: u16 = 16;
const SAMPLERATE: u32 = 44_100;
const CHANNELS: u16 = 1;
const BLOCKALIGN: u16 = BITDEPTH / 2;
const BYTERATE: u32 = SAMPLERATE * BITDEPTH as u32 / 8;
const FORMAT: u16 = 1; // WAVE_FORMAT_PCM
const CHUNKSIZE: u32 = 16;
const DURATION: u8 = 2;
const FREQUENCY: f64 = 442.;

pub fn beep(filename: String) -> Result<()> {
    let (mut output_file, pos_cksize) = gen_output_file(filename)?;

    // Data
    output_file.write_all("data".as_bytes())?;
    let pos_data_placeholder = output_file.stream_position()?;
    output_file.write_all("----".as_bytes())?;
    let pos_data_start = output_file.stream_position()?;
    // generate some sine wave
    let amplitude: f64 = 0.5;
    let offset: f64 = 2. * PI * FREQUENCY / (SAMPLERATE as f64);
    let mut angle: f64 = 0.;
    let samples_required: u64 = SAMPLERATE as u64 * DURATION as u64;

    let mut sample: f64;
    let mut sample_to_write: i16;
    let max_amplitude: f64 = 2f64.powi((BITDEPTH - 1).into()) - 1.;

    for _ in 1..samples_required {
        sample = amplitude * angle.sin();
        angle += offset;
        sample_to_write = (sample * max_amplitude) as i16;
        output_file.write_all(&sample_to_write.to_le_bytes())?;
    }
    let mut pos_end = output_file.stream_position()?;

    let chunk_size_data: u32 = (pos_end - pos_data_start) as u32;
    if chunk_size_data % 2 != 0 {
        output_file.write_all(&[0x00])?;
        pos_end = output_file.stream_position()?;
    }
    output_file.seek(SeekFrom::Start(pos_data_placeholder))?;

    output_file.write_all(&chunk_size_data.to_le_bytes())?;

    output_file.seek(SeekFrom::Start(pos_cksize))?;
    let chunk_size_header: u32 = (pos_end - 8) as u32;
    output_file.write_all(&chunk_size_header.to_le_bytes())?;

    output_file.sync_all()?;

    Ok(())
}
