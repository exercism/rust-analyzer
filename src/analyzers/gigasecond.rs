use crate::{
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        Analyze,
    },
    Result,
};
use super::comments::GeneralComment;

use syn::File;

pub struct GigasecondAnalyzer;

macro_rules! good {
 ($find:expr => $result:expr) => {
    |src| { if src.contains($find) { Some((1, $result.to_string() )) } else { None }}

    };
}

macro_rules! bad {
 ($find:expr => $result:expr) => {
    |src| { if src.contains($find) { Some((-1, $result.to_string() )) } else { None }}

    };
}

macro_rules! missing {
 ($find:expr => $result:expr) => {
    |src| { if !src.contains($find) { Some((-1, $result.to_string() )) } else { None }}

    };
}

pub static LINTS: &[fn(&str) -> Option<(i32, String)>] = &[
    good!("1_000_000_000" => "Excellent use of '_' to keep the number readable."),

    good!("start + Duration::seconds(1_000_000_000)" => "Perfect!"),

    bad!(".pow(" => "Rather than using `.pow`, rust number literals can have `_` in them to make them more readable, for example: `1_000`. (This also avoids the cast)"),

    bad!("Utc.timestamp(" => "This could be simplified to `start + Duration::seconds`."),

    bad!("start.add(" => "Did you know that the `+` operator works for DateTime?"),

    bad!("use chrono::{DateTime, Utc, Duration};" => "(don't forget to use cargo fmt)"),

    missing!("_" => "Did you know that rust number literals can have `_` in them to make them more readable? For example: `1_000`")
];

impl Analyze for GigasecondAnalyzer {
    fn analyze(&self, _solution_ast: &File, solution_raw: &str) -> Result<AnalysisOutput> {
        const METHOD: &str = "after(";
        // Check if the function is present, before
        // running any lints on the solution.
        let source = solution_raw;
        if source.contains(METHOD) {
            return Ok(AnalysisOutput::new(
                AnalysisStatus::Disapprove,
                vec![GeneralComment::SolutionFunctionNotFound.to_string()],
            ));
        }

        let mut analysis: Vec<(i32, String)> = LINTS.iter().filter_map(|lint| lint(solution_raw)).collect();
        let score: i32 = analysis.iter().map(|(score, _)| score).sum();
        analysis.sort_by_key(|(score, _)| score);

        let status = if score > 1 {
            AnalysisStatus::Approve
        } else if score < -1 {
            AnalysisStatus::Disapprove
        } else {
            AnalysisStatus::ReferToMentor
        };
        Ok(AnalysisOutput{
            status,
            comments: analysis.map(|(_, comment)| comment)
        })
    }
}