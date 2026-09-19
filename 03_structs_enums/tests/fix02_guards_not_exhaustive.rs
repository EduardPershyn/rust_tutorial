//! FIX 02: guards and exhaustiveness.
//! Every i32 IS covered below, but the compiler doesn't analyze `if` guards, so it can't prove it.
//! Rewrite the arms with RANGE PATTERNS so the compiler can check it. No `if` guards, no `_` arm.
//! Run: cargo test --test fix02_guards_not_exhaustive

fn bucket(n: i32) -> &'static str {
    match n {
        ..0 => "negative",
        0 => "zero",
        1..=9 => "digit",
        10.. => "big",
    }
}

#[test]
fn buckets() {
    assert_eq!(bucket(i32::MIN), "negative");
    assert_eq!(bucket(-1), "negative");
    assert_eq!(bucket(0), "zero");
    assert_eq!(bucket(1), "digit");
    assert_eq!(bucket(9), "digit");
    assert_eq!(bucket(10), "big");
    assert_eq!(bucket(i32::MAX), "big");
}
