use crate::AnalyzerResult;
use std::{fs, path::Path};

#[derive(Copy, Clone)]
pub enum AnalysisStatus {
    ApproveAsOptimal,
    ApproveWithComment,
    DisapproveWithComment,
    ReferToMentor,
}

pub struct AnalysisOutput {
    status: AnalysisStatus,
    comments: Vec<String>,
}

impl AnalysisOutput {
    pub fn new(status: AnalysisStatus, comments: Vec<String>) -> Self {
        Self { status, comments }
    }

    pub fn write(&self, file_path: &Path) -> AnalyzerResult<()> {
        let analysis_content = format!(
            "{{\n    \"status\": \"{}\",\n    \"comments\": []\n}}",
            Into::<&str>::into(self.status)
        );

        fs::write(file_path, analysis_content)?;

        Ok(())
    }
}

impl From<AnalysisStatus> for &str {
    fn from(status: AnalysisStatus) -> Self {
        use AnalysisStatus::*;
        match status {
            ApproveAsOptimal => "approve_as_optimal",
            ApproveWithComment => "approve_with_comment",
            DisapproveWithComment => "disapprove_with_comment",
            ReferToMentor => "refer_to_mentor",
        }
    }
}
