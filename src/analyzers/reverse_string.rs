use crate::analyzers::{
    output::{AnalysisOutput, AnalysisStatus},
    Analyze,
};
use crate::AnalyzerResult;
use std::{
    fmt::{self, Display},
    path::Path,
};

enum ReverseStringComments {
    SuggestDoingBonusExercise,
}

impl Display for ReverseStringComments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReverseStringComments::*;
        write!(
            f,
            "{}",
            match self {
                SuggestDoingBonusExercise => "rust.reverse_string.suggest_doing_bonus_exercise",
            }
        )
    }
}

pub struct ReverseStringAnalyzer;

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, dir: &Path) -> AnalyzerResult<AnalysisOutput> {
        Ok(AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![]))
    }
}

mod test {
    use super::ReverseStringComments::*;
    use super::*;
    use crate::analyzers::Analyze;
    use std::path::Path;

    const TEST_ANALYZER: ReverseStringAnalyzer = ReverseStringAnalyzer;

    fn test_analyzer_output(solution_path: &str, expected: AnalysisOutput) {
        // TODO: Find a way to compare without the unwrap.
        assert_eq!(
            TEST_ANALYZER.analyze(Path::new(solution_path)).unwrap(),
            expected
        )
    }

    #[test]
    fn analyze_returns_approve_with_comment_1() {
        test_analyzer_output(
            "snippets/reverse-string/optimal_with_comment_1",
            AnalysisOutput::new(
                AnalysisStatus::ApproveWithComment,
                vec![SuggestDoingBonusExercise.to_string()],
            ),
        );
    }

    #[test]
    fn analyze_returns_approve_with_comment_2() {
        test_analyzer_output(
            "snippets/reverse-string/optimal_with_comment_2",
            AnalysisOutput::new(
                AnalysisStatus::ApproveWithComment,
                vec![SuggestDoingBonusExercise.to_string()],
            ),
        );
    }

    #[test]
    fn analyze_returns_approve_as_optimal_1() {
        test_analyzer_output(
            "snippets/reverse-string/optimal_1",
            AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
        );
    }

    #[test]
    fn analyze_returns_approve_as_optimal_2() {
        test_analyzer_output(
            "snippets/reverse-string/optimal_2",
            AnalysisOutput::new(AnalysisStatus::ApproveAsOptimal, vec![]),
        );
    }
}
