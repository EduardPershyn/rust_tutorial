// Run: cargo run --example structs_methods

// ---------- a struct with named fields ----------
#[derive(Debug, Clone, PartialEq)]
struct Account {
    owner: String,
    balance: u64,
}

impl Account {
    // Associated function (no `self`): the conventional "constructor".
    fn new(owner: &str) -> Self {
        Self { owner: owner.to_string(), balance: 0 }
    }

    // &self: read-only access.
    fn owner(&self) -> &str {
        &self.owner // return a view into the field, don't give the String away
    }

    // &mut self: may modify.
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    // Two different accounts borrowed mutably at the same time: fine.
    fn transfer_to(&mut self, other: &mut Account, amount: u64) -> bool {
        if self.balance < amount {
            return false;
        }
        self.balance -= amount;
        other.balance += amount;
        true
    }

    // self: consumes the account. The caller can't use it afterwards.
    fn close(self) -> u64 {
        println!("  closing {}'s account", self.owner);
        self.balance
    }
}

// A second impl block for the same type is allowed.
impl Account {
    fn is_empty(&self) -> bool {
        self.balance == 0
    }
}

// ---------- tuple structs & the newtype pattern ----------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Gwei(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Wei(u128);

impl Gwei {
    fn to_wei(self) -> Wei {
        Wei(self.0 as u128 * 1_000_000_000)
    }
}

fn fee(gas: u64, price: Gwei) -> Wei {
    Wei(gas as u128 * price.to_wei().0)
}

// ---------- unit struct & Default ----------
#[derive(Debug)]
struct Marker;

#[derive(Debug, Default)]
struct Config {
    verbose: bool,
    retries: u32,
    name: String,
}

fn main() {
    // construction and field access
    let mut alice = Account::new("alice");
    alice.deposit(100); // auto-borrow: Account::deposit(&mut alice, 100)
    println!("owner={} balance={}", alice.owner(), alice.balance);

    let mut bob = Account { owner: String::from("bob"), balance: 5 };
    let ok = alice.transfer_to(&mut bob, 30);
    println!("transfer ok={ok}: alice={} bob={}", alice.balance, bob.balance);
    // alice.transfer_to(&mut alice, 1); // ❌ E0499: can't borrow alice mutably twice → no self-transfer bugs

    // Debug printing
    println!("{alice:?}");
    println!("{bob:#?}"); // pretty, multi-line

    // derive(PartialEq) gives ==, derive(Clone) gives .clone()
    let copy = alice.clone();
    println!("alice == copy: {}", alice == copy);

    // field-init shorthand and struct update syntax
    let owner = String::from("carol");
    let carol = Account { owner, balance: 0 };
    let rich_carol = Account { balance: 1_000, ..carol }; // moves carol.owner into rich_carol
    // println!("{}", carol.owner);    // ❌ E0382: moved by the update syntax above
    println!("carol.balance={} (Copy field: still usable)", carol.balance);
    println!("rich_carol={rich_carol:?}, is_empty={}", rich_carol.is_empty());

    // consuming method
    let final_balance = alice.close();
    println!("final balance {final_balance}");
    // alice.deposit(1);               // ❌ E0382: alice was consumed by close()

    // newtypes
    let price = Gwei(30);
    let cost = fee(21_000, price);
    println!("fee for a plain transfer at {price:?}: {cost:?}");
    // fee(21_000, 30);                // ❌ E0308: expected `Gwei`, found integer
    // fee(21_000, Wei(30));           // ❌ E0308: expected `Gwei`, found `Wei`
    println!("size_of::<Gwei>()={} (same as u64: zero cost)", std::mem::size_of::<Gwei>());

    // unit struct and Default
    let m = Marker;
    let cfg = Config { verbose: true, ..Config::default() };
    println!("{m:?} {cfg:?}");
    println!("verbose={} retries={} name={:?}", cfg.verbose, cfg.retries, cfg.name);
}
