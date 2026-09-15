//! FIX 03: more than one mutable borrow.
//! Make it compile. Don't change what the asserts check.
//! Run: cargo test --test fix03_two_mutable

fn deposit(balance: &mut u64, amount: u64) {
    *balance += amount;
}

#[test]
fn two_deposits() {
    let mut balance = 100;
    let a = &mut balance;
    deposit(a, 10);

    let b = &mut balance;
    deposit(b, 20);
    assert_eq!(balance, 130);
}

#[test]
fn append_to_itself() {
    // push_str needs `&mut s` while its argument reads `&s`, and the buffer may reallocate
    // under the reader. Here making a copy is unavoidable, so `.clone()` is fine.
    let mut s = String::from("ab");
    s.push_str(&s.clone());
    assert_eq!(s, "abab");
}
