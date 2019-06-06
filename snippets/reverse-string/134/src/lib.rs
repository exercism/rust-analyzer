pub fn reverse(input: &str) -> String {
    if input == "" {
      return String::from("");
    }
    let mut reversed = String::new();
    for char in input.chars().rev() {
      reversed.push(char);
    }
    return reversed
}
