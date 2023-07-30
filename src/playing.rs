use crate::{backbone::SAMPLE_RATE, processing::MixedRoot};
use anyhow::Result;
use rodio::{buffer::SamplesBuffer, OutputStream, Source};

pub fn play(mix: &MixedRoot) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    mix.iter()
        .try_for_each(|(album, album_data)| -> Result<()> {
            album_data
                .1
                .iter()
                .try_for_each(|(track, track_data)| -> Result<()> {
                    let source = SamplesBuffer::new(1, SAMPLE_RATE, track_data.to_owned());
                    let duration = source.total_duration().unwrap();
                    stream_handle.play_raw(source)?;
                    println!("{} - {}", album, track);
                    std::thread::sleep(duration);
                    Ok(())
                })?;
            Ok(())
        })?;
    Ok(())
}
