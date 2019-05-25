pub fn reverse(input: &str) -> String {
	let mut reverse = String::new();

	for c in input.chars().rev() {
		reverse.push(c);
	}

	reverse
}
