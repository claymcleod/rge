//! Analyses supported by the `rge` command line tool.

use noodles::fasta::Record;

use self::{ndetect::NRegionDetectionAnalysis, par::PseudoAutosomalRegionAnalysis};

pub mod ndetect;
pub mod par;

/// Gets all analyses supported by the `rge` command line tool.
pub fn get_analyses() -> Vec<Box<dyn Analysis>> {
    vec![
        Box::new(PseudoAutosomalRegionAnalysis::default()),
        Box::new(NRegionDetectionAnalysis::default()),
    ]
}

/// An analysis supported by `rge`.
pub trait Analysis {
    /// Name of the analysis.
    fn name(&self) -> &'static str;

    /// Processes a record contained within the FASTA file.
    fn process(&mut self, record: &Record) -> anyhow::Result<()>;

    /// Once processing of all records has concluded, this method will be called
    /// to allow analyses to aggregate their results.
    fn postprocess(&mut self) -> anyhow::Result<()>;

    /// Prints the report for an analysis to stdout.
    fn print_report(&self) -> anyhow::Result<()>;
}
