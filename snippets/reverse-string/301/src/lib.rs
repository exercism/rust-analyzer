pub fn reverse(input: &str) -> String {
    let mut return_string: String = "".to_string();
    return_string = input.chars().rev().collect::<String>();
   // for number in (0..(input.len())).rev() {
   //    return_string.push_str(&input[number..].chars().next().unwrap().to_string())
   //}
    return return_string;
}
