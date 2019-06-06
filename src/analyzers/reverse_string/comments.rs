//! #comments
//! This module contains comments and their string representation for the reverse-string analyzer.

use std::fmt::{self, Display};

pub enum ReverseStringComment {
    SuggestRemovingExternCrate,
    SuggestDoingBonusTest,
}

impl Display for ReverseStringComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReverseStringComment::*;
        write!(
            f,
            "{}",
            match self {
                SuggestRemovingExternCrate => "rust.reverse_string.suggest_removing_extern_crate",
                SuggestDoingBonusTest => "rust.reverse_string.suggest_doing_bonus_test",
            }
        )
    }
}
