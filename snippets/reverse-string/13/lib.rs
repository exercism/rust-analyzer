pub fn reverse(input: &str) -> String {
    let mut out = String::new();
    let mut input = input.to_string();
    while input.len()>0 {
        out.push(input.pop().unwrap());
    }
    return out;
}
