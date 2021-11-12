use crate::prelude::*;

const LITERAL_WITH_UNDERSCORE_USED: &str = "rust.general.literal_with_underscore_used";
const PLUS_OP_USED: &str = "rust.gigasecond.plus_operator_used";

const PREFER_LITERAL: &'static str = "rust.gigasecond.literal_instead_of_pow";
const ADD_DURATION: &'static str = "rust.gigasecond.add_duration";
const USE_PLUS_OP: &'static str = "rust.gigasecond.use_plus_operator";

const SORT_USE_ELEMENTS: &'static str = "rust.general.sort_use_elements";
const CHECKED_OVERFLOW: &str = "rust.gigasecond.checked_overflow";
const USE_LITERAL_WITH_UNDERSCORE: &str = "rust.general.use_literal_with_underscore";
const DIRECT_RETURN_EXPRESSION: &str = "rust.general.direct_return_expression";
const PERFECT: &str = "rust.general.perfect";

pub static LINTS: &[Lint] = &[
    good!("1_000_000_000" => LITERAL_WITH_UNDERSCORE_USED),
    good!("start +" => PLUS_OP_USED),
    good!("start + Duration::seconds(1_000_000_000)" => PERFECT),
    bad!(".pow(" => PREFER_LITERAL),
    bad!("Utc.timestamp(" => ADD_DURATION),
    bad!("start.add(" => USE_PLUS_OP),
    note!("use chrono::{DateTime, Utc, Duration};" => SORT_USE_ELEMENTS),
    good!("checked_add_signed" => CHECKED_OVERFLOW),
    bad_if_missing!("_" => USE_LITERAL_WITH_UNDERSCORE),
    good_if_missing!(";" => DIRECT_RETURN_EXPRESSION),
];

pub struct GigasecondAnalyzer;

impl Analyze for GigasecondAnalyzer {
    fn analyze(&self, _solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput> {
        self.run_lints("after(", solution_raw, LINTS, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::output::AnalysisStatus;

    fn test_analyzer_output(solution_ast: &File, solution_raw: &str, expected: AnalysisOutput) {
        assert_eq!(
            GigasecondAnalyzer.analyze(solution_ast, solution_raw).ok(),
            Some(expected)
        )
    }

    #[test]
    fn approve_and_well_done_for_using_underscores_in_literal() {
        let solution = r#"
use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let td = Duration::seconds(1_000_000_000);

    start + td
}
            "#;
        test_analyzer_output(
            &syn::parse_str::<File>(solution).unwrap(),
            solution,
            AnalysisOutput::new(
                AnalysisStatus::Approve,
                vec![
                    LITERAL_WITH_UNDERSCORE_USED.to_string(),
                    PLUS_OP_USED.to_string(),
                ],
            ),
        );
    }
}
