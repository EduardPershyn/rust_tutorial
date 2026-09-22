//! Exercise 02: a custom error enum with Display, and operations that must be atomic.
//! Run: cargo test --lib ex02
//! Not allowed in this file: `unwrap()` and `expect()` (outside the tests).

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BankError {
    ZeroAmount,
    AccountNotFound(String),
    InsufficientFunds { needed: u64, available: u64 },
    SameAccount,
}

impl fmt::Display for BankError {
    /// Messages, exactly:
    ///   ZeroAmount                                     → "amount must be positive"
    ///   AccountNotFound("bob")                         → "account not found: bob"
    ///   InsufficientFunds { needed: 50, available: 20 } → "insufficient funds: need 50, have 20"
    ///   SameAccount                                    → "cannot transfer to the same account"
    /// Hint: `match self { … => write!(f, "…") }` (see the README recipe)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for BankError {}

/// A bank holding (owner, balance) pairs. Create one with `Bank::default()`.
///
/// Hint: write a private helper that finds an account's INDEX,
/// `fn index_of(&self, name: &str) -> Result<usize, BankError>`,
/// then read or change `self.accounts[i].1`. (Holding two `&mut` into the same Vec at once
/// doesn't compile, but two indexes are just numbers.)
#[derive(Debug, Default)]
pub struct Bank {
    accounts: Vec<(String, u64)>,
}

impl Bank {
    /// Open an account with a zero balance. Opening an existing name again changes nothing.
    pub fn open(&mut self, name: &str) {
        todo!()
    }

    /// The current balance.
    pub fn balance(&self, name: &str) -> Result<u64, BankError> {
        todo!()
    }

    /// Add money. Checks, in this order: ZeroAmount, AccountNotFound.
    pub fn deposit(&mut self, name: &str, amount: u64) -> Result<(), BankError> {
        todo!()
    }

    /// Take money out and return the NEW balance.
    /// Checks, in this order: ZeroAmount, AccountNotFound, InsufficientFunds.
    pub fn withdraw(&mut self, name: &str, amount: u64) -> Result<u64, BankError> {
        todo!()
    }

    /// Move money between accounts. ATOMIC: on any error, no balance may change.
    /// Checks, in this order: SameAccount, ZeroAmount, `from` not found, `to` not found,
    /// InsufficientFunds.
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), BankError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bank() -> Bank {
        let mut b = Bank::default();
        b.open("ann");
        b.open("bob");
        b
    }

    #[test]
    fn messages() {
        assert_eq!(BankError::ZeroAmount.to_string(), "amount must be positive");
        assert_eq!(BankError::AccountNotFound("bob".to_string()).to_string(), "account not found: bob");
        let low = BankError::InsufficientFunds { needed: 50, available: 20 };
        assert_eq!(low.to_string(), "insufficient funds: need 50, have 20");
        assert_eq!(BankError::SameAccount.to_string(), "cannot transfer to the same account");
    }

    #[test]
    fn open_and_balance() {
        let mut b = bank();
        assert_eq!(b.balance("ann"), Ok(0));
        assert_eq!(b.balance("eve"), Err(BankError::AccountNotFound("eve".to_string())));
        b.deposit("ann", 10).unwrap();
        b.open("ann"); // already exists: nothing changes
        assert_eq!(b.balance("ann"), Ok(10));
    }

    #[test]
    fn deposit_errors_in_order() {
        let mut b = bank();
        assert_eq!(b.deposit("eve", 0), Err(BankError::ZeroAmount)); // ZeroAmount is checked first
        assert_eq!(b.deposit("eve", 5), Err(BankError::AccountNotFound("eve".to_string())));
    }

    #[test]
    fn withdraw_values() {
        let mut b = bank();
        b.deposit("ann", 100).unwrap();
        assert_eq!(b.withdraw("ann", 30), Ok(70));
        assert_eq!(b.withdraw("ann", 71), Err(BankError::InsufficientFunds { needed: 71, available: 70 }));
        assert_eq!(b.balance("ann"), Ok(70)); // unchanged after the failed withdrawal
        assert_eq!(b.withdraw("ann", 0), Err(BankError::ZeroAmount));
        assert_eq!(b.withdraw("eve", 1), Err(BankError::AccountNotFound("eve".to_string())));
    }

    #[test]
    fn transfer_happy_path() -> Result<(), BankError> {
        // a test can return Result: any `?` failing here fails the test
        let mut b = bank();
        b.deposit("ann", 50)?;
        b.transfer("ann", "bob", 20)?;
        assert_eq!(b.balance("ann")?, 30);
        assert_eq!(b.balance("bob")?, 20);
        Ok(())
    }

    #[test]
    fn transfer_errors_in_order() {
        let mut b = bank();
        b.deposit("ann", 50).unwrap();
        assert_eq!(b.transfer("ghost", "ghost", 0), Err(BankError::SameAccount));
        assert_eq!(b.transfer("ann", "bob", 0), Err(BankError::ZeroAmount));
        assert_eq!(b.transfer("eve", "zed", 5), Err(BankError::AccountNotFound("eve".to_string())));
        assert_eq!(b.transfer("ann", "zed", 5), Err(BankError::AccountNotFound("zed".to_string())));
        assert_eq!(
            b.transfer("ann", "bob", 51),
            Err(BankError::InsufficientFunds { needed: 51, available: 50 })
        );
    }

    #[test]
    fn transfer_is_atomic() {
        let mut b = bank();
        b.deposit("ann", 50).unwrap();
        assert!(b.transfer("ann", "zed", 10).is_err()); // `to` doesn't exist
        assert!(b.transfer("ann", "bob", 60).is_err()); // not enough money
        assert_eq!(b.balance("ann"), Ok(50)); // nothing left ann's account
        assert_eq!(b.balance("bob"), Ok(0));
    }
}
