pub fn reverse(input: &str) -> String {
    let mut ret = String::new();
    for char in input.chars() {
        ret = char.to_string() + &ret;
    }
    ret
}
