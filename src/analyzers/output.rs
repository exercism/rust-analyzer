//! # output
//! This module contains the structures that are necessary to represent the result of the exercise analysis
//! according to the [Exercism automatic mentoring interface](https://github.com/exercism/automated-mentoring-support/blob/master/docs/interface.md)

use crate::Result;
use serde::{Serialize, Serializer};
use std::{
    fmt::{self, Display},
    fs,
    path::Path,
};

/// The status of the exercise analysis.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AnalysisStatus {
    Approve,
    Disapprove,
    ReferToMentor,
}

/// The result of the exercise analysis.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct AnalysisOutput {
    pub status: AnalysisStatus,
    pub comments: Vec<String>,
}

impl AnalysisOutput {
    pub fn new(status: AnalysisStatus, comments: Vec<String>) -> Self {
        Self { status, comments }
    }

    /// Writes self to the `analysis_file_path` as a JSON file.
    pub fn write(&self, analysis_file_path: &Path) -> Result<()> {
        fs::write(analysis_file_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

impl Display for AnalysisStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AnalysisStatus::*;
        write!(
            f,
            "{}",
            match self {
                Approve => "approve",
                Disapprove => "disapprove",
                ReferToMentor => "refer_to_mentor",
            }
        )
    }
}

impl Serialize for AnalysisStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
