pub fn reverse(input: &str) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    chars.reverse();
    let mut reverse_str = String::new();
    for c in chars {
        reverse_str.push(c);
    }
    reverse_str
}
