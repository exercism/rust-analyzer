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
use syn::File;

pub struct ReverseStringAnalyzer;

const OPTIMAL_SOLUTIONS: [&str; 2] = [
    "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }",
    "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }"
];
const OPTIMAL_SOLUTIONS_WITH_EXTERN_CRATE: [&str; 2] = [
    "extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }",
    "extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }"
];
const OPTIMAL_SOLUTIONS_SUGGEST_BONUS: [&str; 2] = [
    "pub fn reverse(input: &str) -> String { input.chars().rev().collect() }",
    "pub fn reverse(input: &str) -> String { input.chars().rev().collect::<String>() }",
];

fn check_known_solutions(solution_ast: &File, known_solutions: &[&str]) -> Option<File> {
    known_solutions
        .iter()
        .filter_map(|solution| syn::parse_str::<File>(solution).ok())
        .find(|ast| ast == solution_ast)
}

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, solution_ast: &File) -> Result<AnalysisOutput> {
        use AnalysisStatus::*;
        Ok(check_known_solutions(&solution_ast, &OPTIMAL_SOLUTIONS)
           .map(|_| AnalysisOutput::new(ApproveAsOptimal, vec![]))
           .or_else(|| {
               check_known_solutions(&solution_ast, &OPTIMAL_SOLUTIONS_WITH_EXTERN_CRATE).map(|_| {
                   AnalysisOutput::new(
                       ApproveWithComment,
                       vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
                   )
               })
           })
           .or_else(|| {
               check_known_solutions(&solution_ast, &OPTIMAL_SOLUTIONS_SUGGEST_BONUS).map(|_| {
                   AnalysisOutput::new(
                       ApproveWithComment,
                       vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
                   )
               })
           })
           .unwrap_or_else(|| AnalysisOutput::new(ReferToMentor, vec![])))
    }
}
