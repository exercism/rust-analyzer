# My Solution to Reverse String problem in Rust

I've solved this using the crate unicode-segmentation, relying for a starting
point on the [documentation here](https://crates.io/crates/unicode-segmentation).

My first effort at a solution did not handle graphemes correctly. I include it below for historical purposes, but please see lib.rs and the passing tests for the actual implementation. I added another test of some cyrillic just for fun.

```
/* Don't do this! */
fn original_broken(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut reversed: Vec<char> = Vec::new();
    for c in chars.iter().rev() {
        reversed.push(*c);
    }
    return reversed.into_iter().collect();
}
```

## Reverse String

Reverse a string

For example:
input: "cool"
output: "looc"

### Bonus

Test your function on this string: `uüu` and see what happens. Try to write a function that properly
reverses this string. Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the
last test, and execute the tests with:
