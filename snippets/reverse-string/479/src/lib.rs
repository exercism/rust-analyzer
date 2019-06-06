pub fn reverse(input: &str) -> String {

    fn inner(mut acc: String,mut remaining: String)->String {
        if remaining.is_empty() {
            return acc;
        }
        acc.push(remaining.pop().unwrap());
        inner(acc, remaining)
    }
    inner( String::new(), input.to_string())
}
