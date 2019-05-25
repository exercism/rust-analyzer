pub fn reverse(input: &str) -> String {
    //I'm not sure if setting the capacity here is helpful?
    //Idea is to allocate all the memory at once - but does push respect that?
    let mut dest = String::with_capacity(input.len());

    for c in input.chars().rev() {
       dest.push(c);
    }

    dest
}
