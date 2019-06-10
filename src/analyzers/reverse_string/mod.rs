pub mod comments;
#[cfg(test)]
mod test;

use crate::{
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        Analyze,
    },
    Result,
};
use comments::ReverseStringComment;
use lazy_static::lazy_static;
use syn::File;

pub struct ReverseStringAnalyzer;

struct PreparedOutput<'a> {
    solution: &'a str,
    output: &'a AnalysisOutput,
}

impl<'a> PreparedOutput<'a> {
    fn new(solution: &'a str, output: &'a AnalysisOutput) -> Self {
        Self { solution, output }
    }
}

lazy_static! {
    static ref OPTIMAL_SOLUTION_OUTPUT: AnalysisOutput =
        AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]);
    static ref OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT: AnalysisOutput = AnalysisOutput::new(
        AnalysisStatus::ApproveWithComment,
        vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
    );
    static ref OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT: AnalysisOutput = AnalysisOutput::new(
        AnalysisStatus::ApproveWithComment,
        vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
    );
    static ref PREPARED_SOLUTION_OUTPUTS: Vec<PreparedOutput<'static>> = vec![
        PreparedOutput::new("use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }", &OPTIMAL_SOLUTION_OUTPUT),
        PreparedOutput::new("use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }", &OPTIMAL_SOLUTION_OUTPUT),
        PreparedOutput::new("pub fn reverse(input: &str) -> String { input.chars().rev().collect() }", &OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT),
        PreparedOutput::new("pub fn reverse(input: &str) -> String { input.chars().rev().collect::<String>() }", &OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT),
        PreparedOutput::new("extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }", &OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT),
        PreparedOutput::new("extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }", &OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT),
    ];
}

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, solution_ast: &File) -> Result<AnalysisOutput> {
        Ok(
            if let Some(output) = PREPARED_SOLUTION_OUTPUTS
                .iter()
                .find(|prepared_output| {
                    syn::parse_str::<File>(prepared_output.solution).unwrap() == *solution_ast
                })
                .map(|prepared_output| prepared_output.output)
                .cloned()
            {
                output
            } else {
                AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![])
            },
        )
    }
}
