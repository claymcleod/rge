use std::fmt::Display;

use anyhow::bail;
use noodles::{
    core::Position,
    fasta::{record::Sequence, Record},
};

use super::Analysis;

//=========================//
// Utility results structs //
//=========================//

#[derive(Default, Debug)]
pub struct PseudoAutosomalScanResult {
    pub start_position: Option<Position>,
    pub ns_until_position: Option<Position>,
    pub same_until_position: Option<Position>,

    pub start_to_ns_len: Option<i64>,
    pub ns_to_same_len: Option<i64>,
}

#[derive(Default, Debug)]
pub struct PairedPseudoAutosomalScanResult {
    pub chr_x: PseudoAutosomalScanResult,
    pub chr_y: PseudoAutosomalScanResult,
}

impl Display for PairedPseudoAutosomalScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chromosome X:\n  ")?;
        write!(f, "Start: {}, ", self.chr_x.start_position.unwrap())?;
        write!(
            f,
            "Ns until: {} (Len: {:>6}), ",
            self.chr_x.ns_until_position.unwrap(),
            self.chr_x.start_to_ns_len.unwrap()
        )?;
        write!(
            f,
            "Same until: {} (Len: {:>6})\n",
            self.chr_x.same_until_position.unwrap(),
            self.chr_x.ns_to_same_len.unwrap()
        )?;

        write!(f, "Chromosome Y:\n  ")?;
        write!(f, "Start: {}, ", self.chr_y.start_position.unwrap())?;
        write!(
            f,
            "Ns until: {} (Len: {:>6}), ",
            self.chr_y.ns_until_position.unwrap(),
            self.chr_y.start_to_ns_len.unwrap()
        )?;
        write!(
            f,
            "Same until: {} (Len: {:>6})\n",
            self.chr_y.same_until_position.unwrap(),
            self.chr_y.ns_to_same_len.unwrap()
        )
    }
}

//================//
// Scan direction //
//================//

pub enum ScanDirection {
    Forward,
    Reverse,
}

//===============//
// Main analysis //
//===============//

#[derive(Default)]
pub struct PseudoAutosomalRegionAnalysis {
    chr_x: Option<Sequence>,
    chr_y: Option<Sequence>,
    forward_results: Option<PairedPseudoAutosomalScanResult>,
    reverse_results: Option<PairedPseudoAutosomalScanResult>,
}

impl PseudoAutosomalRegionAnalysis {
    pub fn scan_for_pseudoautosomal_region(
        &self,
        direction: ScanDirection,
    ) -> anyhow::Result<PairedPseudoAutosomalScanResult> {
        // (1) Set up the scan's position and direction
        let chr_x = self.chr_x.as_ref().unwrap();
        let chr_y = self.chr_y.as_ref().unwrap();

        let (mut x_ptr, mut y_ptr, forward) = match direction {
            ScanDirection::Forward => (1, 1, true),
            ScanDirection::Reverse => (chr_x.len(), chr_y.len(), false),
        };

        let mut result = PairedPseudoAutosomalScanResult::default();
        result.chr_x.start_position = Some(Position::try_from(x_ptr).unwrap());
        result.chr_y.start_position = Some(Position::try_from(y_ptr).unwrap());

        // (2) Detect for non-N base for chrX
        loop {
            let position = Position::try_from(x_ptr)?;
            let base = *chr_x.get(position).unwrap() as char;

            if base != 'N' && base != 'n' {
                break;
            }

            if forward {
                x_ptr += 1;
            } else {
                x_ptr -= 1;
            }
        }

        result.chr_x.ns_until_position = Some(Position::try_from(x_ptr)?);
        result.chr_x.start_to_ns_len =
            Some((usize::from(result.chr_x.start_position.unwrap()) as i64 - x_ptr as i64).abs());

        // (3) Detect for non-N base for chrY
        loop {
            let position = Position::try_from(y_ptr)?;
            let base = *chr_y.get(position).unwrap() as char;

            if base != 'N' && base != 'n' {
                break;
            }

            if forward {
                y_ptr += 1;
            } else {
                y_ptr -= 1;
            }
        }

        result.chr_y.ns_until_position = Some(Position::try_from(y_ptr)?);
        result.chr_y.start_to_ns_len =
            Some((usize::from(result.chr_y.start_position.unwrap()) as i64 - y_ptr as i64).abs());

        // (4) Now, track each chromsome until the nucleotides split
        loop {
            let x_position = Position::try_from(x_ptr)?;
            let y_position = Position::try_from(y_ptr)?;

            let x_char = chr_x.get(x_position).unwrap();
            let y_char = chr_y.get(y_position).unwrap();

            if x_char != y_char {
                break;
            }

            if forward {
                x_ptr += 1;
                y_ptr += 1;
            } else {
                x_ptr -= 1;
                y_ptr -= 1;
            }
        }

        result.chr_x.same_until_position = Some(Position::try_from(x_ptr)?);
        result.chr_y.same_until_position = Some(Position::try_from(y_ptr)?);

        result.chr_x.ns_to_same_len = Some(
            (usize::from(result.chr_x.ns_until_position.unwrap()) as i64 - x_ptr as i64).abs(),
        );
        result.chr_y.ns_to_same_len = Some(
            (usize::from(result.chr_y.ns_until_position.unwrap()) as i64 - y_ptr as i64).abs(),
        );

        Ok(result)
    }
}

impl Analysis for PseudoAutosomalRegionAnalysis {
    fn name(&self) -> &'static str {
        "Pseudoautosomal Region Analysis"
    }

    fn process(&mut self, record: &Record) -> anyhow::Result<()> {
        if record.name() == "chrX" {
            self.chr_x = Some(record.sequence().clone());
        } else if record.name() == "chrY" {
            self.chr_y = Some(record.sequence().clone());
        }

        Ok(())
    }

    fn postprocess(&mut self) -> anyhow::Result<()> {
        // (1) Check to ensure we actually found chrX and chrY
        if self.chr_x.is_none() {
            bail!("we didn't identify chromosome X! Does this genome use accessions instead?");
        } else if self.chr_y.is_none() {
            bail!("we didn't identify chromosome Y! Does this genome use accessions instead?");
        }

        // (2) Scan for the Pseudoautosomal regions from the front and back of these chromosomes
        self.forward_results = Some(self.scan_for_pseudoautosomal_region(ScanDirection::Forward)?);
        self.reverse_results = Some(self.scan_for_pseudoautosomal_region(ScanDirection::Reverse)?);

        Ok(())
    }

    fn print_report(&self) -> anyhow::Result<()> {
        println!("---");
        println!("Pseudoautosomal Region 1");
        println!("{}", self.forward_results.as_ref().unwrap());
        println!();
        println!("---");
        println!("Pseudoautosomal Region 2");
        println!("{}", self.reverse_results.as_ref().unwrap());
        Ok(())
    }
}
