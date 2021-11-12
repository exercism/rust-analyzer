//! #comments
//! This module contains general comments and their string representation that can be used by any analyzer.

use std::fmt::{self, Display};

pub enum GeneralComment {
    SolutionFunctionNotFound,
    SolutionFileNotFound,
    FailedToParseSolutionFile,
}

const GENERAL_COMMENT_PREFIX: &str = "rust.general";

impl Display for GeneralComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GeneralComment::*;
        write!(
            f,
            "{}.{}",
            GENERAL_COMMENT_PREFIX,
            match self {
                SolutionFileNotFound => "solution_file_not_found",
                SolutionFunctionNotFound => "solution_function_not_found",
                FailedToParseSolutionFile => "failed_to_parse_solution_file",
            }
        )
    }
}
