use anyhow::{Context, Result};
mod audio;
mod doc;
mod file;
mod proc;

fn main() -> Result<()> {
    let (input, output) = proc::get_filenames().context("Cannot find the filename in args")?;
    let data = file::read_file(input).context("Cannot process file data")?;
    let serialized = proc::serialize(data).context("Cannot parse text")?;
    dbg!(serialized);
    Ok(())
}
