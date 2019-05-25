pub fn reverse(input: &str) -> String {
    let len = input.chars().count();
    let mut reversed = String::with_capacity(len);

    for c in input.chars() {
        reversed.insert(0, c);
    }

    reversed
}
