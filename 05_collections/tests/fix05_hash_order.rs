//! FIX 05: a test that depends on HashMap iteration order.
//! This compiles, but the test fails (and would fail differently on each run).
//! Make it pass by making the OUTPUT deterministic: change `sorted_names`, not the test.
//! Run: cargo test --test fix05_hash_order (run it a few times)

use std::collections::HashMap;

/// All names in the map, in alphabetical order.
fn sorted_names(scores: &HashMap<String, u32>) -> Vec<String> {
    let mut names = Vec::new();
    for name in scores.keys() {
        names.push(name.clone());
    }
    names
}

#[test]
fn names_are_alphabetical() {
    let mut scores = HashMap::new();
    for name in ["dave", "ann", "carol", "bob", "eve"] {
        scores.insert(name.to_string(), 1);
    }
    assert_eq!(sorted_names(&scores), ["ann", "bob", "carol", "dave", "eve"]);
}
