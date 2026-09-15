//! FIX 02: mutating while a shared borrow is alive.
//! Make it compile. You may reorder statements or change how variables are declared.
//! Don't change what the asserts check.
//! Run: cargo test --test fix02_borrow_then_mutate

#[test]
fn check_then_push() {
    let mut balances = vec![100, 200];

    let first = &balances[0];
    assert_eq!(*first, 100);

    balances.push(300);
    assert_eq!(balances.len(), 3);
}

#[test]
fn remember_old_price() {
    // Reordering can't help here: the old value is needed AFTER the change.
    let mut prices = [10, 20, 30];
    let old_first = prices[0];
    prices[0] = 99;
    assert_eq!(prices[0] - old_first, 89);
}
