pub fn reverse(input: &str) -> String {
    // if input is empty, return empty
    if input.is_empty() {
        String::from("")
    } else {
        // create a reversed iterator for word
        let reverse_chars = input.chars().rev();
        // initialize an empty string of correct size
        let mut reverse = String::with_capacity(input.len());
        // push chars into reverse in reversed order
        for char in reverse_chars {
            reverse.push(char)
        }
        reverse
    }
}
