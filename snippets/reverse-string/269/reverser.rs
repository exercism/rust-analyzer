use std::iter::Rev;
use std::str::Chars;

use unic_normal::StrNormalForm;

/// Reverser struct gets the string slice ant hands out the iterator over reversed string.
///
/// It is handled internally by creating an (owned) String, normalized according to
/// Unicode Normalization Form C (using the [Unic](https://docs.rs/unic-normal/0.9.0/unic_normal/) crate).
/// If the normalization appears to be a no-op (e.g. when the string is already in NFC),
/// the generated string is immediately thrown away without storing in struct, and Reverser uses
/// the fallback implementation without graphemes handling.
///
/// Note that this returns the non-normalized reversed string, i.e. if some grapheme cluster was
/// split into several `char`s in the input, it will be the same in the output.
///
/// # Examples
/// Simple reversing for ASCII string, UTF-8 string and string with grapheme clusters:
/// ```
/// use reverse_string::Reverser;
/// assert_eq!(Reverser::from("testing string").collect::<String>(), "gnirts gnitset");
/// assert_eq!(Reverser::from("фыва").collect::<String>(), "авыф");
/// assert_eq!(Reverser::from("uüu").collect::<String>(), "uüu");
/// ```
///
/// Note that Reverser borrows from the string he's got, so it can't be passed over to somewhere else:
/// ```compile_fail
/// # fn main() { let _ = failed(); }
/// use reverse_string::Reverser;
/// fn failed<'a>() -> Reverser<'a> {
///     let owned = String::from("local string");
///     Reverser::from(owned.as_str())
/// }
/// ```
#[derive(Debug)]
pub struct Reverser<'a> {
    orig: Rev<Chars<'a>>,
    nfc: Option<String>,
    buf: Option<String>, // TODO maybe a [u8; N] would be better (for some N)?..
}

impl<'a> Iterator for Reverser<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some(ref mut buf) = &mut self.buf {
            let buf_char = buf.pop(); // it is guaranteed to be Some
            if buf.len() == 0 {
                self.buf = None
            }
            buf_char
        } else if let Some(ref mut nfc) = &mut self.nfc {
            let nfc_char = nfc.pop();
            let orig_char = self.orig.next();
            if nfc_char == orig_char {
                // this also automatically handles None
                orig_char
            } else {
                // TODO are these unwraps guaranteed not to panic?
                let mut buf = orig_char.unwrap().to_string();
                // iterator is recreated every time - this is intentional, since buffer is recreated too
                while buf.chars().nfc().next() != nfc_char {
                    let orig_char = self.orig.next();
                    buf = orig_char.unwrap().to_string() + &buf;
                }
                // TODO: I don't like these four lines of code, have to refactor later
                buf = buf.chars().rev().collect();
                let buf_char = buf.pop();
                self.buf = Some(buf);
                buf_char
            }
        } else {
            self.orig.next()
        }
    }
}

// TODO: allow to pass in &String too (it's impossible for now)
impl<'a> From<&'a str> for Reverser<'a> {
    fn from(input: &'a str) -> Reverser {
        let nfc: String = input.nfc().collect();
        if nfc == input {
            Reverser {
                orig: input.chars().rev(),
                nfc: None,
                buf: None,
            }
        } else {
            Reverser {
                orig: input.chars().rev(),
                nfc: Some(nfc),
                buf: None,
            }
        }
    }
}
