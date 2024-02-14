use anyhow::Result;
use bppt_wav::{export, play, Channel, Signal, Track};
use clap::{arg, Parser, Subcommand};
use meval::Expr;
use serde_json::from_str;
use std::{fs::read_to_string, str::FromStr};

/// Argument parser entry point
#[derive(Parser)]
#[clap(author, version, about="The BPPT / MML interpreter powered by math", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a directory tree of generated wav files
    Export {
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
    /// Plays a sample track to test a signal expression
    Try {
        /// Signal expression
        #[arg(value_name = "EXPRESSION")]
        expr: String,
        /// Track
        #[arg(value_name = "TRACK")]
        track: Option<String>,
    },
}

impl Cli {
    pub(crate) fn look_what_to_do_and_do_it() -> Result<()> {
        match Self::parse().cmd {
            Command::Export { r#in, out } => export(
                from_str::<Track>(read_to_string(r#in)?.as_str())?.mix()?,
                out,
            ),
            Command::Play { r#in } => {
                play(from_str::<Track>(read_to_string(r#in)?.as_str())?.mix()?)
            }
            Command::Try { expr, track } => play(
                {
                    let mut custom = Track::default();
                    if let Some(s) = track {
                        custom.channels = vec![Channel::new(
                            Signal(Expr::from_str(&expr).expect("can't parse expression")),
                            from_str(&format!(r#"{{"set": "aAbcCdDefFgG", "score": "{}"}}"#, s))?,
                            442.0,
                        )]
                    } else {
                        custom.channels.iter_mut().next().unwrap().signal =
                            Signal(Expr::from_str(&expr).expect("can't parse expression"));
                    };
                    custom
                }
                .mix()?,
            ),
        }
    }
}

fn main() -> Result<()> {
    Cli::look_what_to_do_and_do_it()
}

/* Help for expressions:
 * https://thewolfsound.com/sine-saw-square-triangle-pulse-basic-waveforms-in-synthesis/
 * https://github.com/rekka/meval-rs#supported-expressions
 * https://www.desmos.com/calculator
 * sine with vibrato (FM): sin((2*pi*f*t)-cos(2*pi*8*t))
 * easy sine-like triangular wave: (2/pi)*signum(sin(t))*(t%pi)-signum(sin(t))
 * FM for any 2pi-periodic (sine-like) function: https://www.desmos.com/calculator/hfxv6h1n9n
 */
