pub fn reverse(input: &str) -> String {

	let mut answer = String::new();
	for c in input.chars().rev() {
		answer.push(c);
	} 
    //unimplemented!("Write a function to reverse {}", input);
    answer
}
