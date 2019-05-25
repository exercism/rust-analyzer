pub fn reverse(input: &str) -> String {
    let mut output = String::from("");
    let len = input.len();
    for i in 1..=len {
        output.push(input.chars().nth(len-i).unwrap());
    }
    output
}
