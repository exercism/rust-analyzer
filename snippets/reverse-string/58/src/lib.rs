pub fn reverse(input: &str) -> String {
  input.graphemes().chars().rev().collect()
}
