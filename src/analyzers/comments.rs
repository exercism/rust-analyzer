//! #comments
//! This module contains comments and their string representation used by every analyzer.

use std::fmt::{self, Display};

pub enum ReverseStringComment {
    SuggestDoingBonusTest,
}

pub enum GeneralComment {
    SolutionFileNotFound,
    FailedToParseSolutionFile,
}

impl Display for ReverseStringComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReverseStringComment::*;
        write!(
            f,
            "{}",
            match self {
                SuggestDoingBonusTest => "rust.reverse_string.suggest_doing_bonus_test",
            }
        )
    }
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
