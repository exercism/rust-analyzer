pub fn reverse(input: &str) -> String {
  let input_vec: Vec<char> = input.chars().collect();
  let rev_input: String = input_vec.into_iter().rev().collect();
  return rev_input;
}