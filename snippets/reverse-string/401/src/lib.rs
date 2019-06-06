pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);

	let mut v: Vec<char> = Vec::new();
  //v.push(1);

	for c in input.chars() {
		v.push(c);
	}
	
	v.reverse();
	
	let s: String = v.into_iter().collect();
	return s;
	
}
