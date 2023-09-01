use super::playing::play;
use super::saving::save;
use super::structure::{Root, Signal};
use anyhow::Result;
use clap::{arg, Parser, Subcommand};
use meval::Expr;
use serde_json::from_str;
use std::{fs::read_to_string, str::FromStr};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(super) struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
pub(super) enum Command {
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
    /// Plays a sample track to test a signal expression
    Try {
        /// Signal expression
        #[arg(value_name = "EXPRESSION")]
        expr: String,
    },
}

impl Cli {
    pub(super) fn look_what_to_do_and_do_it() -> Result<()> {
        match Self::parse().cmd {
            Command::Save { r#in, out } => save(
                from_str::<Root>(read_to_string(r#in)?.as_str())?.mix()?,
                out,
            ),
            Command::Play { r#in } => {
                play(from_str::<Root>(read_to_string(r#in)?.as_str())?.mix()?)
            }
            Command::Try { expr } => play(
                {
                    let mut custom = Root::default();
                    custom
                        .0
                        .iter_mut()
                        .next()
                        .unwrap()
                        .1
                        .tracks
                        .iter_mut()
                        .next()
                        .unwrap()
                        .1
                        .channels
                        .iter_mut()
                        .next()
                        .unwrap()
                        .1
                        .signal = Signal(Expr::from_str(&expr)?);
                    custom
                }
                .mix()?,
            ),
        }
    }
}
