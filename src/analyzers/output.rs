use crate::AnalyzerResult;
use serde::{Serialize, Serializer};
use std::{fs, path::Path};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AnalysisStatus {
    ApproveAsOptimal,
    ApproveWithComment,
    DisapproveWithComment,
    ReferToMentor,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct AnalysisOutput {
    status: AnalysisStatus,
    comments: Vec<String>,
}

impl AnalysisOutput {
    pub fn new(status: AnalysisStatus, comments: Vec<String>) -> Self {
        Self { status, comments }
    }

    pub fn write(&self, analysis_file_path: &Path) -> AnalyzerResult<()> {
        fs::write(analysis_file_path, serde_json::to_string_pretty(self)?)?;
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

impl Serialize for AnalysisStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str((*self).into())
    }
}
