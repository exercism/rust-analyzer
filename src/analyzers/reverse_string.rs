use crate::analyzers::{
    output::{AnalysisOutput, AnalysisStatus},
    Analyze,
};
use crate::AnalyzerResult;
use std::path::Path;

pub struct ReverseStringAnalyzer;

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, dir: &Path) -> AnalyzerResult<AnalysisOutput> {
        Ok(AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![]))
    }
}
