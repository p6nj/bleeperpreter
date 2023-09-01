use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) cmd: Command,
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
}
