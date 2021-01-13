use crate::prelude::*;

const LITERALS_WITH_UNDERSCORE: &str =
    "I like the large numeric is formatted with underscores to be more readable.";
const PLUS_OP_USED: &str = "I like this solution directly adds `Duration` to `start` with \
    the `+` operator.";

const PREFER_LITERAL: &'static str = "Rather than using `.pow`, rust number literals can have \
    `_` in them to make them more readable, for example: `1_000`. (This also avoids the cast)";
const SUGGEST_ADD: &'static str = "This could be simplified to `start + Duration::seconds`.";
const SUGGEST_OP: &'static str = "Did you know that the `+` operator works for DateTime?";

const RUST_FMT: &'static str = "A minor style point is that usually \
    `rustfmt` will usually put `use` elements in alphabetical order. In larger programs with \
    a lot of `use` statements it can be helpful to have them ordered alphabetically.";
const GOOD_FOCUS_ON_OVERFLOW: &str = "Good idea considering overflow conditions by using \
    `checked_add_signed` method of `DateTime<Utc>`.";
const HINT_LITERAL_SEPARATERS: &str = "Did you know that rust number literals can have `_` in \
    them to make them more readable? For example: `1_000`";
const GOOD_NO_RETURN_STATEMENT: &str = "I like that the expression is directly returned instead \
    of being set to a binding and returning the binding or using return and a semicolon.";
const PERFECT: &str = "Perfect";

pub static LINTS: &[Lint] = &[
    good!("1_000_000_000" => LITERALS_WITH_UNDERSCORE),
    good!("start +" => PLUS_OP_USED),
    good!("start + Duration::seconds(1_000_000_000)" => PERFECT),
    bad!(".pow(" => PREFER_LITERAL),
    bad!("Utc.timestamp(" => SUGGEST_ADD),
    bad!("start.add(" => SUGGEST_OP),
    note!("use chrono::{DateTime, Utc, Duration};" => RUST_FMT),
    good!("checked_add_signed" => GOOD_FOCUS_ON_OVERFLOW),
    bad_if_missing!("_" => HINT_LITERAL_SEPARATERS),
    good_if_missing!(";" => GOOD_NO_RETURN_STATEMENT),
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
                    LITERALS_WITH_UNDERSCORE.to_string(),
                    PLUS_OP_USED.to_string(),
                ],
            ),
        );
    }
}
