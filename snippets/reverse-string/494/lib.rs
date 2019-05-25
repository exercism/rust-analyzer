pub fn reverse(input: &str) -> String {
  let result = input.get(input.len()..0);
  match result {
    Some(reversed) => String::from(reversed),
    None => String::from(""),
  }
}
