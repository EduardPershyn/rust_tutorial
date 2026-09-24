//! FIX 03: an ignored Result.
//! Solidity devs know this bug: calling something that can fail and never checking the outcome
//! (the classic unchecked `call` return value). Rust warns about it; this file turns that
//! warning into an error. Make it compile by CHECKING the result. Don't silence it with `let _ =`.
//! Run: cargo test --test fix03_unchecked_result

#![deny(unused_must_use)]

fn send(balance: &mut u64, amount: u64) -> Result<(), String> {
    if amount > *balance {
        return Err(format!("need {amount}, have {balance}"));
    }
    *balance -= amount;
    Ok(())
}

#[test]
fn failed_send_is_noticed() {
    let mut balance = 50;
    assert_eq!(send(&mut balance, 80), Err("need 80, have 50".to_string())); // this should fail: assert that it does
    assert_eq!(balance, 50);
}

#[test]
fn successful_send_is_checked() {
    let mut balance = 50;
    assert_eq!(send(&mut balance, 20), Ok(())); // this should succeed: assert that too
    assert_eq!(balance, 30);
}
