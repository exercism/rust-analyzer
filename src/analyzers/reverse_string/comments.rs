//! #comments
//! This module contains comments and their string representation for the reverse-string analyzer.

use std::fmt::{self, Display};

pub enum ReverseStringComment {
    SuggestRemovingExternCrate,
    SuggestDoingBonusTest,
    SolutionFunctionNotFound,
}

const REVERSE_STRING_COMMENT_PREFIX: &str = "rust.reverse-string";

impl Display for ReverseStringComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReverseStringComment::*;
        write!(
            f,
            "{}.{}",
            REVERSE_STRING_COMMENT_PREFIX,
            match self {
                SuggestRemovingExternCrate => "suggest_removing_extern_crate",
                SuggestDoingBonusTest => "suggest_doing_bonus_test",
                SolutionFunctionNotFound => "solution_function_not_found",
            }
        )
    }
}
