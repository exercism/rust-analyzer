pub fn reverse(input: &str) -> String {
    let mut nstr = String::new();
    let mut sstr = input.to_string();
    while let Some(c) = sstr.pop(){
	nstr.push(c);
    }
    nstr
}
