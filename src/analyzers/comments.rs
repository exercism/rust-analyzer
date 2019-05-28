//! #comments
//! This module contains general comments and their string representation that can be used by any analyzer.

use std::fmt::{self, Display};

pub enum GeneralComment {
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
                SolutionFileNotFound => "rust.general.solution_file_not_found",
                FailedToParseSolutionFile => "rust.general.failed_to_parse_solution_file",
            }
        )
    }
}
