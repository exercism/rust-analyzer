use crate::prelude::*;

const LITERALS_WITH_UNDERSCORE: &str =
    "I like the large numeric is formatted with underscores to be more readable.";
const PLUS_OP_USED: &str = "I like this solution directly adds `Duration` to `start` with \
    the `+` operator.";

const PREFER_LITERAL: &'static str = "Rather than using `.pow`, rust number literals can have \
    `_` in them to make them more readable, for example: `1_000`. (This also avoids the cast)";
const SUGGEST_ADD: &'static str = "This could be simplified to `start + Duration::seconds`.";
const SUGGEST_OP: &'static str = "Did you know that the `+` operator works for DateTime?";
pub static LINTS: &[fn(&str) -> Option<(i32, String)>] = &[
    good!("1_000_000_000" => LITERALS_WITH_UNDERSCORE),
    good!("start +" => PLUS_OP_USED),
    good!("start + Duration::seconds(1_000_000_000)" => "Perfect!"),
    bad!(".pow(" => PREFER_LITERAL),
    bad!("Utc.timestamp(" => SUGGEST_ADD),
    bad!("start.add(" => SUGGEST_OP),
    note!("use chrono::{DateTime, Utc, Duration};" => "A minor style point is that usually \
    `rustfmt` will usually put `use` elements in alphabetical order. In larger programs with \
    a lot of `use` statements it can be helpful to have them ordered alphabetically."),
    good!("checked_add_signed" => "Good idea considering overflow conditions by using \
    `checked_add_signed` method of `DateTime<Utc>`."),
    bad_if_missing!("_" => "Did you know that rust number literals can have `_` in them to make \
    them more readable? For example: `1_000`"),
    good_if_missing!(";" => "I like the expression is directly returned instead of being set \
    to a binding and returning the binding or using return and a semicolon."),
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
