//! FIX 01: `?` in a function that doesn't return Result.
//! Make it compile. Change only the function, not the test. (The errors pointing at the tests,
//! like "no method `is_err` for i32", disappear once the function returns the right type.)
//! Run: cargo test --test fix01_question_mark_return

use std::num::ParseIntError;

fn parse_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

#[test]
fn sums() {
    assert_eq!(parse_sum("2", "3"), Ok(5));
    assert!(parse_sum("2", "x").is_err());
}

#[test]
fn error_type_is_parse_int_error() {
    let err: ParseIntError = parse_sum("x", "1").unwrap_err();
    assert_eq!(err.to_string(), "invalid digit found in string");
}
