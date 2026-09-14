//! FIX 01: use after move.
//! Make it compile without `.clone()`. Don't change what the asserts check.
//! Run: cargo test --test fix01_use_after_move
//! Hint: `total_len` only needs to READ the words.

fn total_len(words: Vec<String>) -> usize {
    let mut total = 0;
    for w in words {
        total += w.len();
    }
    total
}

#[test]
fn words_still_usable() {
    let words = vec![String::from("hello"), String::from("world")];
    let n = total_len(words);
    assert_eq!(n, 10);
    assert_eq!(words.len(), 2);
}
