//! FIX 04: an `unwrap()` hiding inside a function that already returns Result.
//! This one compiles, but a test fails with a panic. Make the function RETURN the error instead.
//! Don't use `unwrap`/`expect`, and don't change the tests.
//! Hint: `let Ok(age) = … else { return Err(…); };`
//! Run: cargo test --test fix04_unwrap_in_result_fn

fn parse_age(s: &str) -> Result<u32, String> {
    let age: u32 = s.trim().parse().unwrap();
    if age > 150 {
        return Err(format!("unrealistic age: {age}"));
    }
    Ok(age)
}

#[test]
fn valid() {
    assert_eq!(parse_age("42"), Ok(42));
    assert_eq!(parse_age(" 7 "), Ok(7));
}

#[test]
fn invalid() {
    assert_eq!(parse_age("200"), Err("unrealistic age: 200".to_string()));
    assert_eq!(parse_age("abc"), Err("not a number: abc".to_string()));
    assert_eq!(parse_age("-3"), Err("not a number: -3".to_string()));
}
