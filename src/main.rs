use std::{fs, path::Path};

use anyhow::{bail, Context, Result};
use rust_analyzer::analyze_exercise;

pub fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        bail!("usage:\nrust-analyzer <slug> <solution_dir> <output_dir>");
    }

    let _slug = &args[0];
    let solution_dir = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    if !solution_dir.exists() {
        bail!("invalid path: {}", solution_dir.display());
    }
    if !output_dir.exists() {
        bail!("invalid path: {}", output_dir.display());
    }

    let analysis = analyze_exercise(solution_dir)?;

    let analysis = serde_json::to_string_pretty(&analysis)?;
    fs::write(output_dir.join("analysis.json"), &analysis)
        .context("failed to write to analysis.json")
}
