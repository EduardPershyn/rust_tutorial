//! FIX 01: a new enum variant breaks a match.
//! `Refunded` was just added to `Payment`, and now `label` doesn't compile. Handle the new variant.
//! Don't add a `_` wildcard arm: then the compiler could no longer warn you about the NEXT new variant.
//! Run: cargo test --test fix01_missing_variant

#[derive(Debug, Clone, Copy, PartialEq)]
enum Payment {
    Pending,
    Settled { amount: u64 },
    Failed,
    Refunded { amount: u64 },
}

fn label(p: Payment) -> String {
    match p {
        Payment::Pending => "pending".to_string(),
        Payment::Settled { amount } => format!("settled {amount}"),
        Payment::Failed => "failed".to_string(),
    }
}

#[test]
fn labels() {
    assert_eq!(label(Payment::Pending), "pending");
    assert_eq!(label(Payment::Settled { amount: 5 }), "settled 5");
    assert_eq!(label(Payment::Failed), "failed");
    assert_eq!(label(Payment::Refunded { amount: 5 }), "refunded 5");
}
