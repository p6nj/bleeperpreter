#[allow(dead_code)]
mod json;
#[allow(dead_code)]
mod tags;
#[allow(dead_code)]
mod valid;

use anyhow::{Context, Result};
use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    json_path: String,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    json::parse(args.json_path).context("error parsing json file")?;
    Ok(())
}
