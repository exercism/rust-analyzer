use std::iter::FromIterator;
// use std::iter::Rev;
// use std::str::Chars;
use std::string::String;

pub fn reverse(input: &str) -> String {
    String::from_iter(input.chars().rev())
}
