//! #analyzers
//! This module contains the implementations of the analyzers for the different exercises.
//! Each analyzer is located in the `exercise_slug/mod.rs` file.
//! The tests for each of the analyzer are located in the `exercise_slug/test.rs` file.

// Macros for defining rules
macro_rules! rule {
    ($points:expr, $($find:literal)|+ => $result:expr) => {
        |src: &str| {if $(src.contains($find))||+ {
            Some(($points, $result.to_string()))
        } else {
            None
        }}
    };
}

macro_rules! not_rule {
    ($points:expr, $($find:literal)|+ => $result:expr) => {
        |src: &str| {if $(!src.contains($find))&&+ {
            Some(($points, $result.to_string()))
        } else {
            None
        }}
    };
}

#[macro_export]
macro_rules! good {
    ($($find:literal)|+ => $result:expr) => {
        rule!(1, $($find)|+ => $result)
    };
}

#[macro_export]
macro_rules! bad {
    ($($find:literal)|+ => $result:expr) => {
        rule!(-1, $($find)|+ => $result)
    };
}

/// Notes are neither good nor bad, and don't change the score.
#[macro_export]
macro_rules! note {
    ($($find:literal)|+ => $result:expr) => {
        rule!(0, $($find)|+ => $result)
    };
}

#[macro_export]
macro_rules! good_if_missing {
    ($($find:literal)|+ => $result:expr) => {
        not_rule!(1, $($find)|+ => $result)
    };
}

#[macro_export]
macro_rules! bad_if_missing {
    ($($find:literal)|+ => $result:expr) => {
        not_rule!(-1, $($find)|+ => $result)
    };
}

#[macro_export]
macro_rules! note_if_missing {
    ($($find:literal)|+ => $result:expr) => {
        not_rule!(0, $($find)|+ => $result)
    };
}

pub type Lint = fn(&str) -> Option<(i32, String)>;

pub mod comments;

pub mod clock;
pub mod gigasecond;
pub mod reverse_string;

pub mod output;
use crate::analyzers::comments::GeneralComment;
use crate::Result;
use output::{AnalysisOutput, AnalysisStatus};
pub use reverse_string::ReverseStringAnalyzer;
use syn::File;

/// This trait contains the analysis logic for the exercise.
/// Should be implemented by every exercise analyzer.
pub trait Analyze {
    /// Tries to analyze the solution provided.
    fn analyze(&self, solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput>;

    /// method_hint is a substring to indicate that this is the right exercise.
    fn run_lints(
        &self,
        method_hint: &str,
        solution_raw: &str,
        lints: &[fn(&str) -> Option<(i32, String)>],
        pass_threshold: i32,
    ) -> Result<AnalysisOutput> {
        if !solution_raw.contains(method_hint) {
            Ok(AnalysisOutput::new(
                AnalysisStatus::Disapprove,
                vec![GeneralComment::SolutionFunctionNotFound.to_string()],
            ))
        } else {
            let analysis: Vec<(i32, String)> =
                lints.iter().filter_map(|lint| lint(solution_raw)).collect();
            let score: i32 = analysis.iter().map(|(score, _)| score).sum();

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
}
