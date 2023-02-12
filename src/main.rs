use anyhow::{Context, Result};

mod audio;
mod cereal;
mod file;
mod proc;
mod structs;

fn main() -> Result<()> {
    let (input, output) = proc::get_filenames().context("Cannot find file names in args")?;
    let data = file::read(input).context("Cannot process file data")?;
    let serialized = cereal::serialize(data).context("Cannot parse text")?;
    println!("{serialized}");
    // let length = serialized.channels.len() as u16;
    file::write(output, proc::render(serialized)?, 1)?;
    Ok(())
}
