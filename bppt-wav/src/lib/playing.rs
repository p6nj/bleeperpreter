use crate::{mixing::Samples, structure::SAMPLE_RATE};
use anyhow::Result;
use rodio::{buffer::SamplesBuffer, OutputStream, Source};

/// Play an entire album, printing the name of each track as it plays. Uses [`rodio`](https://docs.rs/rodio) for the playback.
pub fn play(mix: Samples) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SamplesBuffer::new(1, SAMPLE_RATE, mix);
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source)?;
    std::thread::sleep(duration);
    Ok(())
}
