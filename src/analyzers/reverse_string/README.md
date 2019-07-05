# reverse-string analyzer
This is the source code for the `reverse-string` analyzer.

## approve

```rust
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
```

```rust
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
```

The following two cases are approved with the `rust.reverse_string.suggest_removing_extern_crate` comment:

```rust
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
```

```rust
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
```

The following two cases are approved with the `rust.reverse_string.suggest_doing_bonus_test` comment:

```rust
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
```

```rust
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
```

## disapprove

If the solution does not contain a properly defined `reverse` function it will be disapproved with the `eust.reverse_string.solution_function_not_found` comment:

```rust
fn main() {
    println!("Test");
}
```

## refer_to_mentor

Every other solution.
