pub mod errors;
use crate::errors::AnalyzerError;
use std::{fs, path::Path};
use AnalysisStatus::*;
use ExerciseType::*;

enum ExerciseType {
    ReverseString,
    Unknown,
}

enum AnalysisStatus {
    ApproveAsOptimal,
    ApproveWithComment,
    DisapproveWithComment,
    ReferToMentor,
}

pub type AnalyzerResult<T> = Result<T, AnalyzerError>;

impl From<&str> for ExerciseType {
    fn from(input: &str) -> Self {
        match input {
            "reverse-string" => ReverseString,
            _ => Unknown,
        }
    }
}

impl From<AnalysisStatus> for &str {
    fn from(status: AnalysisStatus) -> Self {
        match status {
            ApproveAsOptimal => "approve_as_optimal",
            ApproveWithComment => "approve_with_comment",
            DisapproveWithComment => "disapprove_with_comment",
            ReferToMentor => "refer_to_mentor",
        }
    }
}

fn write_analysis_result(
    exercise_dir_path: &Path,
    status: AnalysisStatus,
    _comments: &[&str],
) -> AnalyzerResult<()> {
    let analysis_file_path = exercise_dir_path.join("analysis.json");

    let analysis_content = format!(
        "{{\n    \"status\": \"{}\",\n    \"comments\": []\n}}",
        Into::<&str>::into(status)
    );

    fs::write(analysis_file_path, analysis_content)?;

    Ok(())
}

fn run_analysis(dir_path: &Path, exercise_type: ExerciseType) -> AnalyzerResult<()> {
    use syn::Item::*;

    let exercise_content = fs::read_to_string(dir_path.join("src").join("lib.rs"))?;

    let parsed_exercise = syn::parse_file(&exercise_content)?;

    let item = &parsed_exercise.items[0];

    if let Fn(func) = item {
        println!("Func name: {}", func.ident);

        let expr = &func.block.stmts[0];

        dbg!(expr);
    }

    Ok(())
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

    run_analysis(&exercise_dir_path, exercise_type)?;

    write_analysis_result(&exercise_dir_path, ReferToMentor, &[""])
}
