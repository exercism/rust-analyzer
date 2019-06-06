
pub fn reverse(input: &str) -> String {
  let mut reversed_mem : Vec<char> = Vec::new();
  for c in input.chars() {
    reversed_mem.insert(0, c)
  }
  reversed_mem.into_iter().collect()
}
