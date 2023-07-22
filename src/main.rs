#[allow(dead_code)]
mod backbone;
#[allow(dead_code)]
mod jsons;
// #[allow(dead_code)]
// mod processing;
#[allow(dead_code)]
mod tags;

use std::path::Path;

use anyhow::{Context, Result};
use backbone::Root;
use clap::Parser;
use wave_stream::read_wav_from_file_path;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    json_path: String,
}

fn main() -> Result<()> {
    // let args = Arguments::parse();
    // jsons::parse(args.json_path).context("error parsing json file")?;
    // jsons::parse(Path::new("json pocs").join("poc1.json"))?
    let parsed: Root = jsons::parse(Path::new("json pocs").join("poc1.json"))?.try_into()?;
    let wav = read_wav_from_file_path(Path::new("sound").join("piano.wav").as_path())?;
    Ok(())
}
