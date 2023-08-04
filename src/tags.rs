use anyhow::{Error, Result};
use id3::{Tag, TagLike, Version};
pub(crate) fn apply(path: &str, album: &str, album_artist: &str) -> Result<(), Error> {
    let mut tag = Tag::new();
    tag.set_album(album);
    tag.set_album_artist(album_artist);
    tag.write_to_wav_path(path, Version::Id3v24)?;
    Ok(())
}
