pub fn reverse(input: &str) -> String {
    // This passes the tests but since String supports unicode I don't think it does "the right thing"[TM].
    return input.chars().rev().collect();
}
