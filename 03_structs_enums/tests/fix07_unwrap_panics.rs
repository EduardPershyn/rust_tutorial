//! FIX 07: `unwrap()` on None panics.
//! This one compiles, but the test fails. Make it pass without the PANICKING methods `unwrap()` and
//! `expect()`, and without `match`. (Non-panicking relatives like `unwrap_or…` are fine: see the README.)
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
    //if let Some(x) = find_price(item) { x } else { 0 }
    //find_price(item).unwrap_or_default()
    let Some(x) = find_price(item) else {
        return 0;
    };
    x
}

#[test]
fn prices() {
    assert_eq!(price_or_zero("apple"), 3);
    assert_eq!(price_or_zero("caviar"), 0);
}
