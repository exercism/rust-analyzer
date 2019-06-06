pub fn reverse(input: &str) -> String {
    let reverse_word: String = input
        .chars()
        .rev()
        .collect();

        reverse_word
}
