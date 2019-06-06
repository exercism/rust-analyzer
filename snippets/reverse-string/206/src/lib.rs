pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    input
        .chars()
        .fold(String::new(), |acc, x| format!("{}{}", x.to_string(), acc))
}
