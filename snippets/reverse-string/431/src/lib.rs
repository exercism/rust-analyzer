pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
	let output = input.chars().rev().collect::<String>();
	output
}
