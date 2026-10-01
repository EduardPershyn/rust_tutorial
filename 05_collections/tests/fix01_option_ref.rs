//! FIX 01: `get` on a map returns a REFERENCE inside an Option.
//! Make it compile. Change only `total_score`, not the test.
//! Hint: see the README's "get returns a reference" line; `.copied()` and `.unwrap_or(…)` help.
//! Run: cargo test --test fix01_option_ref

use std::collections::HashMap;

fn total_score(scores: &HashMap<String, u32>, names: &[&str]) -> u32 {
    let mut total = 0;
    for name in names {
        let score: u32 = scores.get(*name).copied().unwrap_or(0);
        total += score;
    }
    total
}

#[test]
fn totals() {
    let mut scores = HashMap::new();
    scores.insert("ann".to_string(), 10);
    scores.insert("bob".to_string(), 7);

    assert_eq!(total_score(&scores, &["ann", "bob"]), 17);
    assert_eq!(total_score(&scores, &["ann", "eve"]), 10); // unknown names count as 0
    assert_eq!(total_score(&scores, &[]), 0);
}
