use super::*;
use crate::analyzers::reverse_string::comments::ReverseStringComment;
use syn::File;

fn test_analyzer_output(solution_ast: &File, expected: AnalysisOutput) {
    assert_eq!(
        ReverseStringAnalyzer.analyze(solution_ast).ok(),
        Some(expected)
    )
}

#[test]
fn analyze_returns_approve_with_comment_suggest_remove_extern_crate_1() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }",
        )
        .unwrap(),
        AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve_with_comment_suggest_remove_extern_crate_2() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }",
        )
        .unwrap(),
        AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve_with_comment_suggest_bonus_1() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "pub fn reverse(input: &str) -> String { input.chars().rev().collect() }",
        )
        .unwrap(),
        AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve_with_comment_suggest_bonus_2() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "pub fn reverse(input: &str) -> String { input.chars().rev().collect::<String>() }",
        )
        .unwrap(),
        AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    );
}

#[test]
fn analyze_returns_approve() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }",
        ).unwrap(),
        AnalysisOutput::new(AnalysisStatus::Approve, vec![]),
    );
}

#[test]
fn analyze_returns_approve_2() {
    test_analyzer_output(
        &syn::parse_str::<File>(
            "use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }",
        ).unwrap(),
        AnalysisOutput::new(AnalysisStatus::Approve, vec![]),
    );
}
