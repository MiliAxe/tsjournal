use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A simple app to keep your journals sane
#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    /// The directory to use
    #[clap(short, long, env = "TS_DIR")]
    pub dir: Option<PathBuf>,

    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new journal
    New {
        /// The name of the new journal
        #[clap(short, long)]
        title: String,
    },

    /// Edit an existing journal
    Edit {},

    /// Print the content of an existing journal
    Print {},
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
