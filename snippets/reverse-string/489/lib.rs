pub fn reverse(input: &str) -> String {
    let mut result: String = String::new();
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();

    for c in chars {
        result.push(c);
    }

    return result;
}
