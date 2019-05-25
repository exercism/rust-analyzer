pub fn reverse(input: &str) -> String {
    let mut string = String::new();
    input.chars().rev().for_each(|ch| {
        string.push(ch);
    });
    string
}
