pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().rev().collect();
    let mut res = String::new();
    for c in chars {
        res.push(c);
    }
    res
}
