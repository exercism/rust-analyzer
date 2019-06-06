pub mod analyzers;
pub mod errors;
use analyzers::{
    comments::GeneralComment,
    output::{AnalysisOutput, AnalysisStatus},
    Analyze, ReverseStringAnalyzer,
};
use errors::AnalyzerError;
use std::{fs, path::Path};

pub type Result<T> = std::result::Result<T, AnalyzerError>;

/// Given the `slug` str, return the appropriate analyzer
/// or an error, if there is no analyzer implemented for the `slug`.
fn get_analyzer(slug: &str) -> Result<&dyn Analyze> {
    match slug {
        "reverse-string" => Ok(&ReverseStringAnalyzer),
        _ => Err(AnalyzerError::InvalidSlugError(slug.to_string())),
    }
}

/// Analyzes the solution at the `solution_dir` directory, using the analyzer,
/// the implementation of which depends on the `slug` argument. Writes the
/// result of the analysis to the `solution_dir/analysis.json` file.
pub fn analyze_exercise(slug: &str, solution_dir: &str) -> Result<()> {
    let solution_dir_path = Path::new(solution_dir);
    if !solution_dir_path.exists() {
        return Err(AnalyzerError::InvalidPathError(solution_dir.to_string()));
    }
    let solution_file_path = solution_dir_path.join("lib.rs");
    let analysis_output = if !solution_file_path.exists() {
        // Solution file does not exist => refer to mentor.
        AnalysisOutput::new(
            AnalysisStatus::ReferToMentor,
            vec![GeneralComment::FailedToParseSolutionFile.to_string()],
        )
    } else if let Ok(solution_ast) = syn::parse_file(&fs::read_to_string(solution_file_path)?) {
        // Solution file exists and can be parsed by syn => run analysis
        get_analyzer(slug)?.analyze(&solution_ast)?
    } else {
        // Solution file could not be parsed by syn => refer to mentor
        AnalysisOutput::new(
            AnalysisStatus::ReferToMentor,
            vec![GeneralComment::FailedToParseSolutionFile.to_string()],
        )
    };
    analysis_output.write(&solution_dir_path.join("analysis.json"))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::analyze_exercise;
    use crate::errors::AnalyzerError;

    #[test]
    fn analyze_exercise_returns_error_for_the_unknown_slug() {
        match analyze_exercise("unknown-slug", "snippets/reverse-string/optimal_1") {
            Err(AnalyzerError::InvalidSlugError(_)) => {},
            _ => panic!("analyze_exercise must return the InvalidSlugError variant if the wrong slug is provided"),
        }
    }

    #[test]
    fn analyze_exercise_returns_error_for_the_invalid_solution_dir() {
        match analyze_exercise("reverse-string", "/some/random/path") {
            Err(AnalyzerError::InvalidPathError(_)) => {},
            _ => panic!("analyze_exercise must return the InvalidPathError variant if the invalid exercise directory is provided"),
        }
    }
}
