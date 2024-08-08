use std::{path::Path, process::Command};

use anyhow::{Context, Result};
use serde::Serialize;

pub fn analyze_exercise(solution_dir: &Path) -> Result<AnalysisOutput> {
    let clippy_output = Command::new("cargo")
        .arg("clippy")
        .arg("--quiet") // suppress compilation progress output
        .arg("--manifest-path")
        .arg(solution_dir.join("Cargo.toml").display().to_string())
        .output()
        .context("failed to run clippy")?
        .stderr;
    let clippy_output =
        String::from_utf8(clippy_output).context("failed to parse clippy output to UTF-8")?;

    let comments = clippy_output
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(Into::into)
        .map(AnalysisComment::new)
        .collect();
    Ok(AnalysisOutput { comments })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AnalysisOutput {
    // summary: String, // optional in spec
    comments: Vec<AnalysisComment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AnalysisComment {
    comment: WebsiteCopyPointer,
    params: ClippyParams,
    r#type: AnalysisType,
}

impl AnalysisComment {
    fn new(clippy_msg: String) -> Self {
        Self {
            comment: WebsiteCopyPointer::GeneralClippy,
            params: ClippyParams { clippy_msg },
            r#type: AnalysisType::Informative,
        }
    }
}

/// Must reference a comment in the website-copy repo.
/// A dot (`.`) represents a file path separator.
/// https://github.com/exercism/website-copy/tree/main/analyzer-comments/rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
enum WebsiteCopyPointer {
    #[serde(rename = "rust.general.clippy")]
    GeneralClippy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ClippyParams {
    clippy_msg: String,
}

/// This enum is defined for the sake of completeness and clarity about the
/// analyzer interface. However, only the "informative" type is used for
/// messages generated by clippy. This is because clippy is quite aggressive
/// and we would risk inconveniencing our users if we pushed them to fix every
/// single clippy warning.
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum AnalysisType {
    Essential,
    Actionable,
    Informative,
    Celebratory,
}
