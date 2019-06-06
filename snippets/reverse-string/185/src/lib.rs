use std::string::String;

pub fn reverse(input: &str) -> String {
    input.to_owned().chars().rev().collect()
}