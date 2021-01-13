//! #comments
//! This module contains general comments and their string representation that can be used by any analyzer.

use std::fmt::{self, Display};

pub enum GeneralComment {
    SolutionFunctionNotFound,
    SolutionFileNotFound,
    FailedToParseSolutionFile,
}

impl Display for GeneralComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GeneralComment::*;
        write!(
            f,
            "{}",
            match self {
                SolutionFileNotFound => "No lib.rs file was submitted?",
                SolutionFunctionNotFound =>
                    "The solution method could not be found. \
                Have given methods been renamed?",
                FailedToParseSolutionFile => "The solution could not be parsed.",
            }
        )
    }
}
