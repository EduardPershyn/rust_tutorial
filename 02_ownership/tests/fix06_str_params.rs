//! FIX 06: overly specific parameter type.
//! Make it compile by changing ONE thing. Don't change the test.
//! Run: cargo test --test fix06_str_params

fn shout(s: &String) -> String {
    s.to_uppercase()
}

#[test]
fn accepts_any_string_like() {
    let owned = String::from("rust");
    assert_eq!(shout(&owned), "RUST");
    assert_eq!(shout("hi"), "HI");
    assert_eq!(shout(&owned[1..3]), "US");
}
