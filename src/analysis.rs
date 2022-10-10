use noodles::fasta::Record;

use self::par::PseudoAutosomalRegionAnalysis;

pub mod par;

pub fn get_analyses() -> Vec<Box<dyn Analysis>> {
    vec![Box::new(PseudoAutosomalRegionAnalysis::default())]
}
pub trait Analysis {
    fn name(&self) -> &'static str;
    fn process(&mut self, record: &Record) -> anyhow::Result<()>;
    fn postprocess(&mut self) -> anyhow::Result<()>;
    fn print_report(&self) -> anyhow::Result<()>;
}
