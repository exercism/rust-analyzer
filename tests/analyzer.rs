use rayon::prelude::*;
use rust_analyzer::{
    analyze_exercise,
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        reverse_string::comments::ReverseStringComment,
    },
};
use std::{fs, path::Path};

const REVERSE_STRING_SLUG: &str = "reverse-string";

fn check_analysis_json(solution_dir_path: &Path, expected: &AnalysisOutput) {
    let analysis_json_path = solution_dir_path.join("analysis.json");
    assert!(analysis_json_path.exists());
    let analysis_json_content = fs::read_to_string(&analysis_json_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read the analysis.json file at path {}",
            analysis_json_path.display()
        )
    });
    let analysis_json_output: serde_json::Value = serde_json::from_str(&analysis_json_content)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to deserialize the content of the analysis.json file at path {}",
                analysis_json_path.display()
            )
        });
    assert_eq!(
        analysis_json_output,
        serde_json::to_value(expected).unwrap()
    );
}

macro_rules! analyzer_test_case {
    ($test_case_name:ident(slug=$slug:expr, snippet_dir=$snippet_dir:expr, expected_output=$expected_output:expr)) => {
        #[test]
        fn $test_case_name() {
            let solution_dir_path = Path::new("snippets").join($slug).join($snippet_dir);
            assert!(analyze_exercise($slug, solution_dir_path.to_str().unwrap()).is_ok());
            check_analysis_json(&solution_dir_path, &$expected_output);
        }
    };
}

analyzer_test_case!(reverse_string_analyzer_writes_json_approve_1(
    slug = REVERSE_STRING_SLUG,
    snippet_dir = "approve_1",
    expected_output = AnalysisOutput::new(AnalysisStatus::Approve, vec![])
));

analyzer_test_case!(reverse_string_analyzer_writes_json_approve_2(
    slug = REVERSE_STRING_SLUG,
    snippet_dir = "approve_2",
    expected_output = AnalysisOutput::new(AnalysisStatus::Approve, vec![])
));

analyzer_test_case!(
    reverse_string_analyzer_writes_json_approve_with_suggest_removing_extern_crate_comment_1(
        slug = REVERSE_STRING_SLUG,
        snippet_dir = "approve_with_suggest_removing_extern_crate_1",
        expected_output = AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
        )
    )
);

analyzer_test_case!(
    reverse_string_analyzer_writes_json_approve_with_suggest_removing_extern_crate_comment_2(
        slug = REVERSE_STRING_SLUG,
        snippet_dir = "approve_with_suggest_removing_extern_crate_2",
        expected_output = AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
        )
    )
);

analyzer_test_case!(
    reverse_string_analyzer_writes_json_approve_with_suggest_bonus_1(
        slug = REVERSE_STRING_SLUG,
        snippet_dir = "approve_with_suggest_bonus_1",
        expected_output = AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        )
    )
);

analyzer_test_case!(
    reverse_string_analyzer_writes_json_approve_with_suggest_bonus_2(
        slug = REVERSE_STRING_SLUG,
        snippet_dir = "approve_with_suggest_bonus_2",
        expected_output = AnalysisOutput::new(
            AnalysisStatus::Approve,
            vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
        )
    )
);

analyzer_test_case!(
    reverse_string_analyzer_writes_json_disapprove_with_solution_not_found(
        slug = REVERSE_STRING_SLUG,
        snippet_dir = "disapprove_with_solution_not_found",
        expected_output = AnalysisOutput::new(
            AnalysisStatus::Disapprove,
            vec![ReverseStringComment::SolutionFunctionNotFound.to_string()],
        )
    )
);

#[test]
fn reverse_string_analyzer_run_on_every_solution() {
    let snippets_dir = Path::new("snippets").join("reverse-string");
    assert!(snippets_dir.exists());
    assert!(snippets_dir
        .read_dir()
        .expect("Failed to get the directories from the reverse-string snippets directory")
        .collect::<std::io::Result<Vec<fs::DirEntry>>>()
        .unwrap()
        .par_iter()
        .map(|solution_dir| analyze_exercise(
            REVERSE_STRING_SLUG,
            solution_dir.path().to_str().unwrap()
        ))
        .all(|analyze_result| analyze_result.is_ok()))
}
