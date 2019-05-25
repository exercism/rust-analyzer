pub fn reverse(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut v = Vec::new();

    for (i, &item) in bytes.iter().rev().enumerate() {
        v.push(item);
    }
    String::from_utf8(v).unwrap()
}
