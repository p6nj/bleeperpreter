use anyhow::{Error, Result};
use id3::{Tag, TagLike, Version};
pub fn apply(path: &str, album: &str) -> Result<(), Error> {
    let mut tag = Tag::new();
    tag.set_album(album);
    tag.write_to_wav_path(path, Version::Id3v24)?;
    Ok(())
}
