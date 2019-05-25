pub fn reverse(input: &str) -> String {
    let mut ans = String::from("");
    let mut input = String::from(input);
    while !input.is_empty() {
        ans.push(input.pop().unwrap())
    }
    ans
}
