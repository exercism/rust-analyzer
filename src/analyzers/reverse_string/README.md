# reverse-string analyzer
This is the source code for the `reverse-string` analyzer.

## approve_as_optimal

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


## approve_with_comment

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

## disapprove_with_comment

In development

## refer_to_mentor

Every other solution.
