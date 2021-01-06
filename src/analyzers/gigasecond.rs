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

const LITERALS_WITH_UNDERSCORE: &str = "Excellent use of '_' to keep the number readable.";

pub static LINTS: &[fn(&str) -> Option<(i32, String)>] = &[
    good!("1_000_000_000" => LITERALS_WITH_UNDERSCORE),
    good!("start + Duration::seconds(1_000_000_000)" => "Perfect!"),
    bad!(".pow(" => "Rather than using `.pow`, rust number literals can have `_` in them to make them more readable, for example: `1_000`. (This also avoids the cast)"),
    bad!("Utc.timestamp(" => "This could be simplified to `start + Duration::seconds`."),
    bad!("start.add(" => "Did you know that the `+` operator works for DateTime?"),
    bad!("use chrono::{DateTime, Utc, Duration};" => "(don't forget to use cargo fmt)"),
    missing!("_" => "Did you know that rust number literals can have `_` in them to make them more readable? For example: `1_000`"),
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
                AnalysisStatus::ReferToMentor,
                vec![LITERALS_WITH_UNDERSCORE.to_string()],
            ),
        );
    }
}
