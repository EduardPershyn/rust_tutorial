//! FIX 07: `unwrap()` on None panics.
//! This one compiles, but the test fails. Make it pass without `unwrap()`, `expect()` or `match`.
//! Run: cargo test --test fix07_unwrap_panics

fn find_price(item: &str) -> Option<u32> {
    match item {
        "apple" => Some(3),
        "bread" => Some(5),
        _ => None,
    }
}

/// Price of the item, or 0 if we don't sell it.
fn price_or_zero(item: &str) -> u32 {
    find_price(item).unwrap()
}

#[test]
fn prices() {
    assert_eq!(price_or_zero("apple"), 3);
    assert_eq!(price_or_zero("caviar"), 0);
}
