pub fn raindrops(n: u32) -> String {
	let mut newString: String = String::from("");
	if n % 3 == 0 {
		newString += "Pling";
	}
	if n % 5 == 0 {
		newString += "Plang";
	}
	if n % 7 == 0 {
		newString += "Plong";
	}
	if newString.len() == 0 {
		let s: String = n.to_string();
		newString = s;
	}
	return newString;
}
