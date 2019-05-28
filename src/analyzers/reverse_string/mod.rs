#[cfg(test)]
mod test;

use crate::{
    analyzers::{
        comments::{GeneralComment, ReverseStringComment},
        output::{AnalysisOutput, AnalysisStatus},
        Analyze,
    },
    AnalyzerResult,
};
use std::{fs, path::Path};
use syn::File;

pub struct ReverseStringAnalyzer;

const OPTIMAL_SOLUTIONS: [&str; 2] = [
    "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }",
    "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }"
];
const OPTIMAL_SOLUTIONS_WITH_COMMENTS: [&str; 2] = [
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
    fn analyze(&self, solution_dir: &Path) -> AnalyzerResult<AnalysisOutput> {
        use AnalysisStatus::*;

        let solution_file_path = solution_dir.join("lib.rs");
        if !solution_file_path.exists() {
            return Ok(AnalysisOutput::new(
                ReferToMentor,
                vec![GeneralComment::SolutionFileNotFound.to_string()],
            ));
        }
        let solution_ast =
            if let Ok(solution_ast) = syn::parse_file(&fs::read_to_string(solution_file_path)?) {
                solution_ast
            } else {
                return Ok(AnalysisOutput::new(
                    ReferToMentor,
                    vec![GeneralComment::FailedToParseSolutionFile.to_string()],
                ));
            };
        Ok(check_known_solutions(&solution_ast, &OPTIMAL_SOLUTIONS)
            .map(|_| AnalysisOutput::new(ApproveAsOptimal, vec![]))
            .or_else(|| {
                check_known_solutions(&solution_ast, &OPTIMAL_SOLUTIONS_WITH_COMMENTS).map(|_| {
                    AnalysisOutput::new(
                        ApproveWithComment,
                        vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
                    )
                })
            })
            .unwrap_or_else(|| AnalysisOutput::new(ReferToMentor, vec![])))
    }
}
