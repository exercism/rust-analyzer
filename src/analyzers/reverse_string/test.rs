use super::*;
use crate::analyzers::reverse_string::comments::ReverseStringComment;
use std::path::Path;

const SNIPPERTS_PREFIX: &str = "snippets/reverse-string";

fn test_analyzer_output(solution_dir: &Path, expected: AnalysisOutput) {
    assert_eq!(
        ReverseStringAnalyzer.analyze(solution_dir).ok(),
        Some(expected)
    )
}

#[test]
fn analyze_returns_approve_with_comment_1() {
    test_analyzer_output(
        &Path::new(SNIPPERTS_PREFIX).join("optimal_with_comment_1"),
        AnalysisOutput::new(
            AnalysisStatus::ApproveWithComment,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve_with_comment_2() {
    test_analyzer_output(
        &Path::new(SNIPPERTS_PREFIX).join("optimal_with_comment_2"),
        AnalysisOutput::new(
            AnalysisStatus::ApproveWithComment,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve_as_optimal_1() {
    test_analyzer_output(
        &Path::new(SNIPPERTS_PREFIX).join("optimal_1"),
        AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
    );
}

#[test]
fn analyze_returns_approve_as_optimal_2() {
    test_analyzer_output(
        &Path::new(SNIPPERTS_PREFIX).join("optimal_2"),
        AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
    );
}
