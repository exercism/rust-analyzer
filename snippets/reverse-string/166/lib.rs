pub fn reverse(input: &str) -> String {
    let str = input.to_string();
    let mut ret = String::from("");
    for x in str.chars().rev() {
        ret.push(x);
    }
    ret
}
