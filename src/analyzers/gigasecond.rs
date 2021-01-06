use super::comments::GeneralComment;
use crate::{
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        Analyze,
    },
    Result,
};

use syn::File;

pub struct GigasecondAnalyzer;

macro_rules! good {
    ($find:expr => $result:expr) => {
        |src| {
            if src.contains($find) {
                Some((1, $result.to_string()))
            } else {
                None
            }
        }
    };
}

macro_rules! bad {
    ($find:expr => $result:expr) => {
        |src| {
            if src.contains($find) {
                Some((-1, $result.to_string()))
            } else {
                None
            }
        }
    };
}

macro_rules! missing {
    ($find:expr => $result:expr) => {
        |src| {
            if !src.contains($find) {
                Some((-1, $result.to_string()))
            } else {
                None
            }
        }
    };
}

const LITERALS_WITH_UNDERSCORE: &str =
    "I like the large numeric is formatted with underscores to be more readable.";
const PLUS_OP_USED: &str = "I like this solution directly adds `Duration` to `start` with the `+` \
    operator.";

pub static LINTS: &[fn(&str) -> Option<(i32, String)>] = &[
    good!("1_000_000_000" => LITERALS_WITH_UNDERSCORE),
    good!("start +" => PLUS_OP_USED),
    good!("start + Duration::seconds(1_000_000_000)" => "Perfect!"),
    bad!(".pow(" => "Rather than using `.pow`, rust number literals can have `_` in them to make \
    them more readable, for example: `1_000`. (This also avoids the cast)"),
    bad!("Utc.timestamp(" => "This could be simplified to `start + Duration::seconds`."),
    bad!("start.add(" => "Did you know that the `+` operator works for DateTime?"),
    bad!("use chrono::{DateTime, Utc, Duration};" => "A minor style point is that usually `rustfmt` \
    will usually put `use` elements in alphabetical order. In larger programs with a lot of `use` statements it can be helpful to have them ordered alphabetically."),
    bad!("checked_add_signed" => "You don't need to use the `checked_add_signed` method of \
    `DateTime<Utc>` since you have to `unwrap` its output `Option` value anyway. \
    What you can do instead is to use the `Add` implementation for `DateTime<Utc>`, \
    meaning `+` can be directly used between `start` and `Duration::seconds(1_000_000_000)` \
    in order to get the desired return value."),
    missing!("_" => "Did you know that rust number literals can have `_` in them to make \
    them more readable? For example: `1_000`"),
    missing!(";" => "I like the expression is directly returned instead of being set to a binding \
    and returning the binding or using return and a semicolon."),
];

impl Analyze for GigasecondAnalyzer {
    fn analyze(&self, _solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput> {
        const METHOD: &str = "after(";
        // Check if the function is present, before
        // running any lints on the solution.
        let source = solution_raw;
        if !source.contains(METHOD) {
            return Ok(AnalysisOutput::new(
                AnalysisStatus::Disapprove,
                vec![GeneralComment::SolutionFunctionNotFound.to_string()],
            ));
        }

        let mut analysis: Vec<(i32, String)> =
            LINTS.iter().filter_map(|lint| lint(solution_raw)).collect();
        let score: i32 = analysis.iter().map(|(score, _)| score).sum();
        analysis.sort_by_key(|(score, _)| *score);

        let status = if score > 1 {
            AnalysisStatus::Approve
        } else if score < -1 {
            AnalysisStatus::Disapprove
        } else {
            AnalysisStatus::ReferToMentor
        };
        Ok(AnalysisOutput {
            status,
            comments: analysis
                .into_iter()
                .map(|(_, comment)| comment)
                .collect::<Vec<_>>(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                vec![LITERALS_WITH_UNDERSCORE.to_string(), PLUS_OP_USED.to_string()],
            ),
        );
    }
}
