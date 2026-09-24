//! FIX 05: testing that something panics.
//! `split_evenly` panics on purpose when `parts == 0` (a caller bug, like dividing by zero).
//! The second test is supposed to CHECK that panic, but right now the panic just fails it.
//! Don't change the function or the test body: change how the test is DECLARED, so it passes
//! only if the function panics with that exact message.
//! Run: cargo test --test fix05_should_panic

fn split_evenly(total: u64, parts: u64) -> u64 {
    assert!(parts > 0, "parts must be non-zero");
    total / parts
}

#[test]
fn splits() {
    assert_eq!(split_evenly(10, 3), 3);
}

#[test]
#[should_panic(expected = "parts must be non-zero")]
fn zero_parts_panics() {
    split_evenly(10, 0);
}
