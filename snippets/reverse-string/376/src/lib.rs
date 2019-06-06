pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();

    if chars.len() < 1 {
        return String::new();
    }

    let mut start = 0;
    let mut end = chars.len() - 1;
    while start < end {
        chars.swap(start, end);
        start += 1;
        end -= 1;
    }

    let mut output = String::new();
    for c in chars {
        output.push(c);
    }

    return output;
}
