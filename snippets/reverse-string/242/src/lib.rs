pub fn reverse(input: &str) -> String {
    let mut rev = String::from("");
    for c in input.chars().rev() {
    rev.push(c);
}
rev
}
