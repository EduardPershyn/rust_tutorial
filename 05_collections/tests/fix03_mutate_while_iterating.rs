//! FIX 03: changing a collection while iterating over it.
//! Make it compile WITHOUT cloning the whole map. Don't change what the asserts check.
//! Hint: collect the keys you want to change first, then change them in a second loop.
//! (Or look up `values_mut`, which hands you &mut V while walking the map.)
//! Run: cargo test --test fix03_mutate_while_iterating

use std::collections::HashMap;

/// Double every score above 5.
fn boost(scores: &mut HashMap<String, u32>) {
    for (name, score) in &*scores {
        if *score > 5 {
            scores.insert(name.clone(), score * 2);
        }
    }
}

#[test]
fn boosts_high_scores() {
    let mut scores = HashMap::new();
    scores.insert("ann".to_string(), 10);
    scores.insert("bob".to_string(), 3);

    boost(&mut scores);

    assert_eq!(scores.get("ann"), Some(&20));
    assert_eq!(scores.get("bob"), Some(&3));
}
