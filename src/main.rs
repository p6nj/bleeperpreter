#[allow(dead_code)]
mod backbone;
#[allow(dead_code)]
mod jsons;
#[allow(dead_code)]
mod tags;

use anyhow::{Context, Result};
use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    json_path: String,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    jsons::parse(args.json_path).context("error parsing json file")?;
    Ok(())
}
