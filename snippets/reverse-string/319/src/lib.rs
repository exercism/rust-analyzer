pub fn reverse(input: &str) -> String {
    String::from_utf8(input.bytes().rev().collect()).unwrap()
}
