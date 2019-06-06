pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    let mut indices = input.char_indices();

    loop {
        let next = indices.next_back();

        if next.is_none() {
            break;
        }

        let (_, c) = next.unwrap();
        result.push_str(&c.to_string());
    }

    result
}
