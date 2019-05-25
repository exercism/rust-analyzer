use std::fs::read_to_string;
use std::path::Prefix::Verbatim;

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
