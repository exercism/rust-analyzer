//! #analyzers
//! This module contains the implementations of the analyzers for the different exercises.
//! Each analyzer is located in the `exercise_slug/mod.rs` file.
//! The tests for each of the analyzer are located in the `exercise_slug/test.rs` file.

pub mod comments;
pub mod output;
pub mod reverse_string;
use crate::Result;
use output::AnalysisOutput;
pub use reverse_string::ReverseStringAnalyzer;
use syn::File;

/// This trait contains the analysis logic for the exercise.
/// Should be implemented by every exercise analyzer.
pub trait Analyze {
    /// Tries to analyze the solution AST provided by the `solution_ast` argument.
    fn analyze(&self, solution_ast: &File) -> Result<AnalysisOutput>;
}
