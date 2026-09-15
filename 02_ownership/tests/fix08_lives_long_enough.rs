//! FIX 08: a reference outliving its owner.
//! Make it compile. Don't change the assert. There are two different valid fixes. Find both.
//! Run: cargo test --test fix08_lives_long_enough

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

#[test]
fn word_outlives_block() {
    let word;
    {
        let text = String::from("short and sweet");
        word = first_word(&text).to_string();
    }
    assert_eq!(word, "short");
}
