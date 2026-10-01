//! FIX 06: slicing a string by bytes.
//! This compiles, but one test panics on non-ASCII text: "byte index 3 is not a char boundary".
//! Make `first_three` count CHARACTERS instead of bytes, without `unwrap`/`expect`.
//! Run: cargo test --test fix06_char_boundary

/// The first three characters, or the whole string if it's shorter.
fn first_three(s: &str) -> String {
    s.chars().take(3).collect()
}

#[test]
fn ascii() {
    assert_eq!(first_three("hello"), "hel");
    assert_eq!(first_three("hi"), "hi");
    assert_eq!(first_three(""), "");
}

#[test]
fn non_ascii() {
    assert_eq!(first_three("Привіт"), "При");
    assert_eq!(first_three("їжак"), "їжа");
}
