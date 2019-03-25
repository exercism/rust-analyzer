use std::path::Path;
use ExerciseType::*;

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

pub fn analyze_exercise(slug: &str, path: &str) -> Result<(), std::io::Error> {
    let exercise_dir_path = Path::new(path);

    if !exercise_dir_path.exists() {
        println!(
            "The provided path does not exist: {}",
            exercise_dir_path.display()
        );

        return Ok(());
    }

    let exercise_type = ExerciseType::from(slug);

    if let Unknown = exercise_type {
        println!("rust-analyzer does not support the '{}' exercise", slug);

        return Ok(());
    }

    Ok(())
}
