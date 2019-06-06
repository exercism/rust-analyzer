use std::str;

pub fn reverse(input: &str) -> String {
    let output = input.chars().rev().collect();
    output
}
