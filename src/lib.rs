mod analyzers;
pub mod errors;
use analyzers::{Analyze, ReverseStringAnalyzer};
use errors::AnalyzerError;
use std::path::Path;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

fn get_analyzer(slug: &str) -> AnalyzerResult<&dyn Analyze> {
    match slug {
        "reverse-string" => Ok(&ReverseStringAnalyzer),
        _ => Err(AnalyzerError::InvalidTypeError(slug.to_string())),
    }
}

pub fn analyze_exercise(slug: &str, path: &str) -> AnalyzerResult<()> {
    let exercise_dir_path = Path::new(path);
    if !exercise_dir_path.exists() {
        return Err(AnalyzerError::InvalidPathError(path.to_string()));
    }
    get_analyzer(slug)?
        .analyze(&exercise_dir_path)?
        .write(&exercise_dir_path.join("analysis.json"))?;
    Ok(())
}
