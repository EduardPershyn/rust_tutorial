//! FIX 04: indexing a map panics when the key is missing.
//! This compiles, but a test panics. Make both tests pass without `unwrap`/`expect`.
//! Run: cargo test --test fix04_index_panics

use std::collections::HashMap;

/// The price of `item`, or 0 if we don't sell it.
fn price_of(prices: &HashMap<&str, u32>, item: &str) -> u32 {
    prices.get(item).copied().unwrap_or(0)
}

#[test]
fn known_item() {
    let prices = HashMap::from([("apple", 3), ("bread", 5)]);
    assert_eq!(price_of(&prices, "apple"), 3);
}

#[test]
fn unknown_item() {
    let prices = HashMap::from([("apple", 3)]);
    assert_eq!(price_of(&prices, "caviar"), 0);
}
