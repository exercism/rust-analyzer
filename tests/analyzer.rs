use rust_analyzer::{
    analyze_exercise,
    analyzers::{
        comments::ReverseStringComment,
        output::{AnalysisOutput, AnalysisStatus},
    },
};
use std::{fs, path::Path};

const REVERSE_STRING_SLUG: &str = "reverse-string";
const REVERSE_STRING_DIR_PREFIX: &str = "snippets/reverse-string";

fn check_analysis_json(solution_dir: &str, expected: &AnalysisOutput) {
    let analysis_json_path = Path::new(solution_dir).join("analysis.json");
    assert!(analysis_json_path.exists());
    let analysis_json_content = fs::read_to_string(&analysis_json_path).expect(&format!(
        "Failed to read the analysis.json file at path {}",
        analysis_json_path.display()
    ));
    let analysis_json_output: serde_json::Value = serde_json::from_str(&analysis_json_content)
        .expect(&format!(
            "Failed to deserialize the content of the analysis.json file at path {}",
            analysis_json_path.display()
        ));
    assert_eq!(
        analysis_json_output,
        serde_json::to_value(expected).unwrap()
    );
}

#[test]
fn reverse_string_analyzer_writes_json_optimal_1() {
    let solution_dir = format!("{}/{}", REVERSE_STRING_DIR_PREFIX, "optimal_1");
    assert!(analyze_exercise(REVERSE_STRING_SLUG, &solution_dir).is_ok());
    check_analysis_json(
        &solution_dir,
        &AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
    )
}

#[test]
fn reverse_string_analyzer_writes_json_optimal_2() {
    let solution_dir = format!("{}/{}", REVERSE_STRING_DIR_PREFIX, "optimal_2");
    assert!(analyze_exercise(REVERSE_STRING_SLUG, &solution_dir).is_ok());
    check_analysis_json(
        &solution_dir,
        &AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
    )
}

#[test]
fn reverse_string_analyzer_writes_json_optimal_with_comment_1() {
    let solution_dir = format!("{}/{}", REVERSE_STRING_DIR_PREFIX, "optimal_with_comment_1");
    assert!(analyze_exercise(REVERSE_STRING_SLUG, &solution_dir).is_ok());
    check_analysis_json(
        &solution_dir,
        &AnalysisOutput::new(
            AnalysisStatus::ApproveWithComment,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    )
}

#[test]
fn reverse_string_analyzer_writes_json_optimal_with_comment_2() {
    let solution_dir = format!("{}/{}", REVERSE_STRING_DIR_PREFIX, "optimal_with_comment_2");
    assert!(analyze_exercise(REVERSE_STRING_SLUG, &solution_dir).is_ok());
    check_analysis_json(
        &solution_dir,
        &AnalysisOutput::new(
            AnalysisStatus::ApproveWithComment,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        ),
    )
}

#[test]
fn reverse_string_analyzer_run_on_every_solution() {
    let snippets_dir = Path::new("snippets").join("reverse-string");
    assert!(snippets_dir.exists());
    snippets_dir
        .read_dir()
        .expect("Failed to get the directories from the reverse-string snippets directory")
        .for_each(|solution_dir| {
            let solution_dir = solution_dir.unwrap();
            let solution_path = solution_dir.path();
            assert!(analyze_exercise(REVERSE_STRING_SLUG, solution_path.to_str().unwrap()).is_ok());
        })
}
