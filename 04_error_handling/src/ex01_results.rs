//! Exercise 01: Result basics: returning errors, `?`, Option → Result, handling vs propagating.
//! Run: cargo test --lib ex01
//! Not allowed in this file: `unwrap()` and `expect()` (outside the tests).

use std::num::ParseIntError;

/// Parse a percentage like "42%" into 42.
/// Errors, with this exact text:
///   no trailing '%'            → "missing %"
///   the rest isn't a number    → "not a number: <the text before %>"
///   the number is above 100    → "out of range: <n>"
/// Hint: `strip_suffix`, then parse as u32 (so "300%" is out of range, not "not a number").
pub fn parse_percent(s: &str) -> Result<u8, String> {
    todo!()
}

/// Sum of all numbers. Stop at the FIRST invalid item and return its error.
pub fn sum_all(items: &[&str]) -> Result<i64, ParseIntError> {
    todo!()
}

/// Sum of the items that ARE valid numbers, skipping the rest.
/// Returns (sum, number of skipped items). Here errors are handled, not propagated.
pub fn sum_valid(items: &[&str]) -> (i64, usize) {
    todo!()
}

/// Age of the user called `name`. Error text: "unknown user: <name>".
pub fn user_age(users: &[(&str, u32)], name: &str) -> Result<u32, String> {
    todo!()
}

/// Is the user 18 or older? Errors from `user_age` pass through unchanged.
/// Hint: reuse `user_age` with `?`; don't repeat the search.
pub fn is_adult(users: &[(&str, u32)], name: &str) -> Result<bool, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERS: [(&str, u32); 2] = [("ann", 30), ("bob", 12)];

    #[test]
    fn percent_ok() {
        assert_eq!(parse_percent("42%"), Ok(42));
        assert_eq!(parse_percent("0%"), Ok(0));
        assert_eq!(parse_percent("100%"), Ok(100));
    }

    #[test]
    fn percent_errors() {
        assert_eq!(parse_percent("42"), Err("missing %".to_string()));
        assert_eq!(parse_percent("abc%"), Err("not a number: abc".to_string()));
        assert_eq!(parse_percent("-5%"), Err("not a number: -5".to_string()));
        assert_eq!(parse_percent("%"), Err("not a number: ".to_string()));
        assert_eq!(parse_percent("101%"), Err("out of range: 101".to_string()));
        assert_eq!(parse_percent("300%"), Err("out of range: 300".to_string()));
    }

    #[test]
    fn sum_all_values() {
        assert_eq!(sum_all(&["1", "2", "3"]), Ok(6));
        assert_eq!(sum_all(&["-5", "5"]), Ok(0));
        assert_eq!(sum_all(&[]), Ok(0));
    }

    #[test]
    fn sum_all_stops_at_first_error() {
        let err = sum_all(&["1", "x", ""]).unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string"); // the error for "x", not for ""
    }

    #[test]
    fn sum_valid_skips_bad_items() {
        assert_eq!(sum_valid(&["1", "x", "3", "", "-2"]), (2, 2));
        assert_eq!(sum_valid(&[]), (0, 0));
        assert_eq!(sum_valid(&["no", "numbers"]), (0, 2));
    }

    #[test]
    fn user_age_values() {
        assert_eq!(user_age(&USERS, "ann"), Ok(30));
        assert_eq!(user_age(&USERS, "eve"), Err("unknown user: eve".to_string()));
    }

    #[test]
    fn is_adult_values() {
        assert_eq!(is_adult(&USERS, "ann"), Ok(true));
        assert_eq!(is_adult(&USERS, "bob"), Ok(false));
        assert_eq!(is_adult(&USERS, "eve"), Err("unknown user: eve".to_string()));
    }
}
