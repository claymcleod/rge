pub mod analysis;
pub mod cli;

use anyhow::Context;
use clap::Parser;
use cli::Cli;
use noodles::fasta;

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
        analysis.print_report()?;
    }

    Ok(())
}
