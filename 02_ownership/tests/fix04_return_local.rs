//! FIX 04: returning a reference to a local variable.
//! Make it compile. Don't change the test.
//! Run: cargo test --test fix04_return_local

fn make_greeting(name: &str) -> &str {
    let greeting = format!("Hello, {name}!");
    &greeting
}

#[test]
fn greets() {
    assert_eq!(make_greeting("Bob"), "Hello, Bob!");
}
