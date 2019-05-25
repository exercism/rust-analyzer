use std::string::String;

pub fn reverse(input: &str) -> String {
    // x::xs -> reverse xs + "x"
    input
        .to_string()
        .chars()
        .fold("".to_string(), |acc, x| x.to_string() + &acc)
}
