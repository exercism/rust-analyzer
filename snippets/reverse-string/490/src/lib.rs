pub fn reverse(input: &str) -> String {
    let mut chars: Vec<_> = input.chars().collect();

    let len = chars.len();
    for i in 0..(chars.len() / 2) {
        chars.swap(i, len - i - 1);
    }
    chars.iter().collect()
}
