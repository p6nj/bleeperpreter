mod backbone;
mod jsons;
mod playing;
mod processing;
mod saving;

use anyhow::Result;
use backbone::Root;
use clap::Parser;
use saving::save;

#[derive(Debug, Parser)]
#[clap(version, about = "An advanced JSON MML interpreter")]
struct Arguments {
    r#in: String,
    out: String,
}

fn main() -> Result<()> {
    // let args = Arguments::parse();
    let args = Arguments {
        r#in: r".\json pocs\poc.json".to_string(),
        out: r".\sound\".to_string(),
    };
    let parsed: Root = jsons::parse(args.r#in)?.try_into()?;
    let mix = parsed.mix()?;
    save(&mix, args.out)?;
    Ok(())
}

// length is inverted?
// note loss!!

/* Help for expressions:
 * https://thewolfsound.com/sine-saw-square-triangle-pulse-basic-waveforms-in-synthesis/
 * https://github.com/rekka/meval-rs#supported-expressions
 * https://www.desmos.com/calculator
 * sine with vibrato (FM): sin((2*pi*f*t)-cos(2*pi*8*t))
 * easy sine-like triangular wave: (2/pi)*signum(sin(t))*(t%pi)-signum(sin(t))
 * FM for any 2pi-periodic (sine-like) function: https://www.desmos.com/calculator/hfxv6h1n9n
 */
