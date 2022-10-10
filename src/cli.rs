use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The path to the reference genome we are exploring.
    #[arg(value_name = "FASTA")]
    pub src: PathBuf,
}
