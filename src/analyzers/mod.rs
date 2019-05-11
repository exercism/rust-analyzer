pub mod output;
pub mod reverse_string;
use crate::AnalyzerResult;
use output::AnalysisOutput;
pub use reverse_string::ReverseStringAnalyzer;
use std::path::Path;

pub trait Analyze {
    fn analyze(&self, dir: &Path) -> AnalyzerResult<AnalysisOutput>;
}
