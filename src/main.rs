use anyhow::{Context, Result};

mod audio;
mod doc;
mod file;
mod proc;

fn main() -> Result<()> {
    let (input, output) = proc::get_filenames().context("Cannot find file names in args")?;
    let data = file::read(input).context("Cannot process file data")?;
    let serialized = proc::serialize(data).context("Cannot parse text")?;
    // dbg!(&serialized);
    file::write(output, proc::render(serialized)?)?;
    Ok(())
}
