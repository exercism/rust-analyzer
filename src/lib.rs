mod analyzers;
pub mod errors;
use analyzers::{Analyze, ReverseStringAnalyzer};
use errors::AnalyzerError;
use std::path::Path;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

fn get_analyzer(slug: &str) -> AnalyzerResult<&dyn Analyze> {
    match slug {
        "reverse-string" => Ok(&ReverseStringAnalyzer),
        _ => Err(AnalyzerError::InvalidSlugError(slug.to_string())),
    }
}

pub fn analyze_exercise(slug: &str, path: &str) -> AnalyzerResult<()> {
    let solution_dir = Path::new(path);
    if !solution_dir.exists() {
        return Err(AnalyzerError::InvalidPathError(path.to_string()));
    }
    get_analyzer(slug)?
        .analyze(&solution_dir)?
        .write(&solution_dir.join("analysis.json"))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::analyze_exercise;
    use crate::errors::AnalyzerError;

    #[test]
    fn analyze_exercise_returns_error_for_the_unknown_slug() {
        match analyze_exercise("unknown-slug", ".") {
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
