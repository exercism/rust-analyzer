pub fn reverse(input: &str) -> String {
    input
        .chars()             // get the chars of the string slice
        .rev()               // reverse them
        .collect::<String>() // finish the iteration indicating the final type
}
