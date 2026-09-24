//! FIX 06: `?` on an Option inside a function that returns Result.
//! Make it compile. Change only the function, not the tests.
//! Hint: turn the Option into a Result first (see the README's table).
//! Run: cargo test --test fix06_option_in_result

fn initial(name: &str) -> Result<char, String> {
    let first = name.chars().next().ok_or("empty name")?;
    Ok(first.to_ascii_uppercase())
}

#[test]
fn initials() {
    assert_eq!(initial("rust"), Ok('R'));
    assert_eq!(initial(""), Err("empty name".to_string()));
}
