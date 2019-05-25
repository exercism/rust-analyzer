use std::iter::FromIterator;

pub fn reverse(input: &str) -> String {
    String::from_iter(input.chars().rev())
}
