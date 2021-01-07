//! #analyzers
//! This module contains the implementations of the analyzers for the different exercises.
//! Each analyzer is located in the `exercise_slug/mod.rs` file.
//! The tests for each of the analyzer are located in the `exercise_slug/test.rs` file.

// Macros for defining rules
#[macro_export]
macro_rules! good {
    ($find:expr => $result:expr) => {
        |src| {
            if src.contains($find) {
                Some((1, $result.to_string()))
            } else {
                None
            }
        }
    };
}

/// Notes are neither good nor bad, and don't change the score.
#[macro_export]
macro_rules! note {
    ($find:expr => $result:expr) => {
        |src| {
            if src.contains($find) {
                Some((0, $result.to_string()))
            } else {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! bad {
    ($find:expr => $result:expr) => {
        |src| {
            if src.contains($find) {
                Some((-1, $result.to_string()))
            } else {
                None
            }
        }
    };
}
#[macro_export]
macro_rules! missing {
    ($find:expr => $result:expr) => {
        |src| {
            if !src.contains($find) {
                Some((-1, $result.to_string()))
            } else {
                None
            }
        }
    };
}

pub mod comments;

pub mod clock;
pub mod gigasecond;
pub mod reverse_string;

pub mod output;
use crate::Result;
use output::{AnalysisOutput, AnalysisStatus};
pub use reverse_string::ReverseStringAnalyzer;
use syn::File;

/// This trait contains the analysis logic for the exercise.
/// Should be implemented by every exercise analyzer.
pub trait Analyze {
    /// Tries to analyze the solution AST provided by the `solution_ast` argument.
    fn analyze(&self, solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput>;

    fn run_lints(
        &self,
        solution_raw: &str,
        lints: &[fn(&str) -> Option<(i32, String)>],
        pass_threshold: i32,
    ) -> Result<AnalysisOutput> {
        let mut analysis: Vec<(i32, String)> =
            lints.iter().filter_map(|lint| lint(solution_raw)).collect();
        let score: i32 = analysis.iter().map(|(score, _)| score).sum();
        analysis.sort_by_key(|(score, _)| *score);
        analysis.reverse();

        let status = if score > pass_threshold {
            AnalysisStatus::Approve
        } else if score < -1 {
            AnalysisStatus::Disapprove
        } else {
            AnalysisStatus::ReferToMentor
        };

        Ok(AnalysisOutput {
            status,
            comments: analysis
                .into_iter()
                .map(|(_, comment)| comment)
                .collect::<Vec<_>>(),
        })
    }
}
