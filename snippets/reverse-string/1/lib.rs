/// Reverses a given String.
///
/// # Examples
///
/// ```
/// let five = "five";
///
/// assert_eq!("five", reverse("evif"));
/// # fn reverse(input: &str) -> String {
/// # return input.chars().rev().collect::<String>();
/// # }
/// ```
pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect::<String>();
}
