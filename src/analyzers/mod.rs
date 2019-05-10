pub mod output;
pub mod reverse_string;
use crate::AnalyzerResult;
use std::path::Path;
use output::AnalysisOutput;

pub trait Analyze {
    fn analyze(&self, dir: &Path) -> AnalyzerResult<AnalysisOutput>;
}
