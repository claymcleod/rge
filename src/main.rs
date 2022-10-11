//! Binary crate for the Reference Genome Explorer (`rge`).

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

pub mod analysis;
pub mod cli;

use anyhow::Context;
use clap::Parser;
use cli::Cli;
use noodles::fasta;
use tracing::info;

use crate::analysis::get_analyses;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // (1) Set up tracing for logging
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // (2) Process the FASTA file
    let mut reader = fasta::reader::Builder::default()
        .build_from_path(cli.src)
        .with_context(|| "opening FASTA file")?;

    // (3) Process analyses
    let mut analyses = get_analyses();

    for result in reader.records() {
        let record = result?;
        info!("Processing record: {}", record.name());

        for analysis in &mut analyses {
            analysis.process(&record)?;
        }
    }

    // (4) Postprocess analyses
    for analysis in &mut analyses {
        analysis.postprocess()?;
    }

    // (5) Print results
    for analysis in &analyses {
        println!("{}", analysis.name());
        println!("-----");
        println!();
        analysis.print_report()?;
        println!();
    }

    Ok(())
}
