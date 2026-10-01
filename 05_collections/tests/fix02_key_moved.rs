//! FIX 02: inserting into a map MOVES the key.
//! Make it compile. Don't change the asserts.
//! Run: cargo test --test fix02_key_moved

use std::collections::HashMap;

#[test]
fn key_still_usable() {
    let mut owners: HashMap<String, u32> = HashMap::new();
    let name = String::from("ann");

    owners.insert(name.clone(), 1);

    assert_eq!(owners.get(&name), Some(&1));
    assert_eq!(name, "ann");
}
