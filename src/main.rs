mod backbone;
mod jsons;
mod playing;
mod processing;
mod saving;

use anyhow::Result;
use backbone::Root;
use clap::Parser;
#[allow(unused_imports)]
use playing::play;
use saving::save;
use std::path::Path;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    json_path: String,
}

fn main() -> Result<()> {
    // let args = Arguments::parse();
    // jsons::parse(args.json_path).context("error parsing json file")?;
    // jsons::parse(Path::new("json pocs").join("poc1.json"))?
    let parsed: Root = jsons::parse(Path::new("json pocs").join("poc.json"))?.try_into()?;
    let mix = parsed.mix()?;
    save(&mix)?;
    Ok(())
}

// length is inverted?
// note loss!!

/* Help for expressions:
 * https://thewolfsound.com/sine-saw-square-triangle-pulse-basic-waveforms-in-synthesis/
 * https://github.com/rekka/meval-rs#supported-expressions
 * https://www.desmos.com/calculator
 * sine with vibrato (FM): sin((2*pi*f*t)-cos(2*pi*8*t))
 */
