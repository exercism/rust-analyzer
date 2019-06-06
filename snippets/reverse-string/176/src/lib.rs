pub fn reverse(input: &str) -> String {
    let mut ret = String::new();

    for c in input.chars().rev() {
        ret.push(c);
    }

    ret
}
