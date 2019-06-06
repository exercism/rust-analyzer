pub fn reverse(input: &str) -> String {
    let mut reversed_string: String = String::from("");
    for i in input.chars() {
        reversed_string.insert(0, i)
    };
    reversed_string
}
