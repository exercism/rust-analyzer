#[cfg(feature = "grapheme")]
#[path=""]
mod impl_grapheme {
    mod reverser;

    pub use reverser::Reverser;

    /// Function to reverse string with grapheme clusters.
    /// It works internally by converting the input into temporary normalized form
    /// and iterating over two strings in parallel.
    pub fn reverse(input: &str) -> String {
        Reverser::from(input).collect()
    }

}

#[cfg(feature = "grapheme")]
pub use impl_grapheme::*;

#[cfg(not(feature = "grapheme"))]
/// Function to reverse simple string.
/// It doesn't try to handle grapheme clusters and simply reverse string as a sequence of `char`s.
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
