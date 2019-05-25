pub fn reverse(input: &str) -> String {
  let mut yo: Vec<&str> = input.split("").collect();
  yo.reverse();
  yo.join("")
}
