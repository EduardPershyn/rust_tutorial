//! Exercise 01: structs, methods, receivers (&self / &mut self / self), newtypes.
//! Run: cargo test --lib ex01

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub owner: String,
    pub balance: u64,
}

impl Account {
    /// A new account with a zero balance.
    pub fn new(owner: &str) -> Self {
        todo!()
    }

    /// The owner's name, borrowed from the account (no allocation).
    pub fn owner(&self) -> &str {
        todo!()
    }

    /// Add funds.
    pub fn deposit(&mut self, amount: u64) {
        todo!()
    }

    /// Take funds out. If the balance is too low, return false and change nothing.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        todo!()
    }

    /// Move `amount` from this account to `other`. If the balance is too low, return false and
    /// change nothing. (Note: `a.transfer_to(&mut a, 1)` doesn't even compile. Why?)
    pub fn transfer_to(&mut self, other: &mut Account, amount: u64) -> bool {
        todo!()
    }

    /// Close the account: consume it and return what was left on it.
    pub fn close(self) -> u64 {
        todo!()
    }
}

/// Gas price unit. 1 gwei = 1_000_000_000 wei.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gwei(pub u64);

/// The smallest ETH unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wei(pub u128);

impl Gwei {
    /// Convert to wei. Must not overflow for any u64 value.
    pub fn to_wei(self) -> Wei {
        todo!()
    }
}

/// Transaction fee: gas_used × price, in wei.
/// Thanks to the newtypes, `gas_cost(21_000, 30)` or `gas_cost(21_000, Wei(30))` don't compile.
pub fn gas_cost(gas_used: u64, price: Gwei) -> Wei {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account() {
        let a = Account::new("alice");
        assert_eq!(a.owner(), "alice");
        assert_eq!(a.balance, 0);
    }

    #[test]
    fn owner_is_borrowed() {
        let a = Account::new("alice");
        assert_eq!(a.owner().as_ptr(), a.owner.as_ptr()); // a view into the field, not a copy
    }

    #[test]
    fn deposit_and_withdraw() {
        let mut a = Account::new("bob");
        a.deposit(100);
        assert!(a.withdraw(30));
        assert_eq!(a.balance, 70);
        assert!(!a.withdraw(71));
        assert_eq!(a.balance, 70);
        assert!(a.withdraw(70));
        assert_eq!(a.balance, 0);
    }

    #[test]
    fn transfer() {
        let mut a = Account { owner: "a".to_string(), balance: 50 };
        let mut b = Account::new("b");
        assert!(a.transfer_to(&mut b, 20));
        assert_eq!((a.balance, b.balance), (30, 20));
        assert!(!a.transfer_to(&mut b, 31));
        assert_eq!((a.balance, b.balance), (30, 20));
    }

    #[test]
    fn close_returns_balance() {
        let mut a = Account::new("carol");
        a.deposit(5);
        assert_eq!(a.close(), 5);
        // a.deposit(1); // ❌ would not compile: `a` was consumed by close()
    }

    #[test]
    fn gwei_to_wei() {
        assert_eq!(Gwei(1).to_wei(), Wei(1_000_000_000));
        assert_eq!(Gwei(0).to_wei(), Wei(0));
        assert_eq!(Gwei(u64::MAX).to_wei(), Wei(18_446_744_073_709_551_615_000_000_000));
    }

    #[test]
    fn gas_costs() {
        assert_eq!(gas_cost(21_000, Gwei(30)), Wei(630_000_000_000_000));
        assert_eq!(gas_cost(0, Gwei(30)), Wei(0));
    }
}
