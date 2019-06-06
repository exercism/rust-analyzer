pub fn reverse(input: &str) -> String {
    let rev_iter = input.chars().rev();


    rev_iter.collect()
}
