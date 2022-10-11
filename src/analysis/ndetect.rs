//! Detection of N regions within the genome.

use std::{collections::HashMap, fmt::Display};

use noodles::core::Position;

use super::Analysis;

/// A region of the genome that contains all N hardmasked pseudonucleobases. The
/// end of the range is inclusive (i.e., `[start, end]`).
#[derive(Debug)]
pub struct NRegion {
    /// Name of the sequence upon which this region sits.
    pub sequence_name: String,

    /// Start position for the region (inclusive).
    pub start: Position,

    /// End position for the region (inclusive).
    pub end: Position,

    /// Size of the region.
    pub dist: usize,
}

impl Display for NRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Sequence Name: {}, ", self.sequence_name)?;
        write!(f, "Start: {}, ", self.start)?;
        write!(f, "End: {}, ", self.end)?;
        write!(f, "Length: {} }}", self.dist)
    }
}

/// Main struct for the N Region detection analysis.
#[derive(Default)]
pub struct NRegionDetectionAnalysis {
    /// N regions detected by the analysis.
    pub regions: Vec<NRegion>,

    /// Sequence names as they were seen by the analysis. This helps us to print
    /// out results in the same sequence name order that they were seen.
    pub sequence_names: Vec<String>,

    /// HashMap containing the total number of Ns in the sequence (if it has
    /// been computed yet).
    pub total_ns: Option<HashMap<String, usize>>,
}

impl Analysis for NRegionDetectionAnalysis {
    fn name(&self) -> &'static str {
        "N Region Detection"
    }

    fn process(&mut self, record: &noodles::fasta::Record) -> anyhow::Result<()> {
        let mut region_start: Option<Position> = None;
        let sequence_name = String::from(record.name());

        for ptr in 1..record.sequence().len() {
            let ptr_position = Position::try_from(ptr)?;
            let ptr_nucleobase = *record.sequence().get(ptr_position).unwrap() as char;

            if ptr_nucleobase == 'N' || ptr_nucleobase == 'n' {
                if region_start.is_none() {
                    region_start = Some(ptr_position);
                }
            } else if let Some(start) = region_start {
                let end = Position::try_from(ptr - 1)?;
                let dist = usize::from(end) - usize::from(start);

                let region = NRegion {
                    sequence_name: sequence_name.clone(),
                    start,
                    end,
                    dist,
                };

                self.regions.push(region);
                region_start = None;
            }
        }

        self.sequence_names.push(sequence_name);
        Ok(())
    }

    fn postprocess(&mut self) -> anyhow::Result<()> {
        let mut total_ns: HashMap<_, usize> = HashMap::new();
        for region in &self.regions {
            *total_ns.entry(region.sequence_name.clone()).or_default() += region.dist;
        }

        self.total_ns = Some(total_ns);
        Ok(())
    }

    fn print_report(&self) -> anyhow::Result<()> {
        if let Some(total_ns) = &self.total_ns {
            println!("Summary table:");
            for name in &self.sequence_names {
                let this_seq_ns = total_ns.get(name).unwrap_or(&0);
                println!("{}\t{}", name, this_seq_ns);
            }

            println!();
            for region in &self.regions {
                println!("{}", region);
            }
        }

        Ok(())
    }
}
