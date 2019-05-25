pub fn reverse(input: &str) -> String {
    let mut rev: Vec<char>  = vec![];
    let input: Vec<char> = input.chars().collect();
    while rev.len() != input.len() {
        rev.push('x')
    }
    for (n, &word) in input.iter().enumerate() {
        rev[(input.len() - n - 1) as usize] = word;
    };
    rev.into_iter().collect()
}
