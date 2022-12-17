use anyhow::{Context, Result};
mod doc;
mod file;
mod proc;

fn main() -> Result<()> {
    let filename = proc::get_filename().context("Cannot find the filename in args")?;
    let data = file::read_file(filename).context("Cannot process file data")?;
    let serialized = proc::serialize(data).context("Cannot parse text")?;
    dbg!(serialized);
    Ok(())
}
