use std::string::String;

pub fn reverse(input: &str) -> String {
    let mut reversed_string = String::new();
    for c in input.chars().rev() {
        reversed_string.push(c);
    }
    reversed_string
}
