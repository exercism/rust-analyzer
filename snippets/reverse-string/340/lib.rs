pub fn reverse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    let mut i = input.len();
    for c in input.chars() {
        i -= 1;
        res.insert(i, c);
    }
    res
}
