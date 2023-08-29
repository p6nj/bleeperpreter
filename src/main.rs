#![feature(iterator_try_reduce)]

mod mixing;
mod playing;
mod saving;
mod structure;

use anyhow::Result;
use clap::{Parser, Subcommand};
use playing::play;
use saving::save;
use serde_json::from_str;
use std::fs::read_to_string;
use structure::Root;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a directory tree of generated wav files
    Save {
        /// JSON album path
        #[arg(value_name = "JSON_FILE")]
        r#in: String,
        /// Root of the generated folder structure
        #[arg(value_name = "EXPORT_FOLDER")]
        out: String,
    },
    /// Just play generated albums and tracks ~in order~ (hopefully)
    Play {
        /// JSON album path
        #[arg(value_name = "JSON_FILE")]
        r#in: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.cmd {
        Command::Save { r#in, out } => save(
            from_str::<Root>(read_to_string(r#in)?.as_str())?.mix()?,
            out,
        ),
        Command::Play { r#in } => play(from_str::<Root>(read_to_string(r#in)?.as_str())?.mix()?),
    }
}

// note loss!!

/* Help for expressions:
 * https://thewolfsound.com/sine-saw-square-triangle-pulse-basic-waveforms-in-synthesis/
 * https://github.com/rekka/meval-rs#supported-expressions
 * https://www.desmos.com/calculator
 * sine with vibrato (FM): sin((2*pi*f*t)-cos(2*pi*8*t))
 * easy sine-like triangular wave: (2/pi)*signum(sin(t))*(t%pi)-signum(sin(t))
 * FM for any 2pi-periodic (sine-like) function: https://www.desmos.com/calculator/hfxv6h1n9n
 */
