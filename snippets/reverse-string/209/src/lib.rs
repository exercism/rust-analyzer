use std::iter::{Iterator};

pub fn reverse(input: &str) -> String {
    let s = String::from(input);
    s.chars().rev().collect()
}
