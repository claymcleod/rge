//! Command line arguments and parsing utilities.

use std::path::PathBuf;

use clap::Parser;

/// Command line arguments for the `rge` command line tool.
#[derive(Parser)]
pub struct Cli {
    /// The path to the reference genome we are exploring.
    #[arg(value_name = "FASTA")]
    pub src: PathBuf,
}
