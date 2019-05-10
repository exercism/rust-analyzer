mod analyzers;
pub mod errors;
use analyzers::Analyze;
use errors::AnalyzerError;
use std::path::Path;
use ExerciseType::*;

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

enum ExerciseType {
    ReverseString,
    Unknown,
}

impl From<&str> for ExerciseType {
    fn from(input: &str) -> Self {
        match input {
            "reverse-string" => ReverseString,
            _ => Unknown,
        }
    }
}

pub fn analyze_exercise(slug: &str, path: &str) -> AnalyzerResult<()> {
    let exercise_dir_path = Path::new(path);
    if !exercise_dir_path.exists() {
        return Err(AnalyzerError::InvalidPathError(path.to_string()));
    }

    let exercise_type = ExerciseType::from(slug);
    if let Unknown = exercise_type {
        return Err(AnalyzerError::InvalidTypeError(slug.to_string()));
    }

    let analyzer = match exercise_type {
        ReverseString => analyzers::reverse_string::ReverseStringAnalyzer,
        _ => unreachable!(),
    };
    analyzer
        .analyze(&exercise_dir_path)?
        .write(&exercise_dir_path.join("analysis.json"))?;

    Ok(())
}
