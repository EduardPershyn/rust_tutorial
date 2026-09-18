// Run: cargo run --example enums_option

// ---------- an enum with data ----------
#[derive(Debug, Clone, PartialEq)]
enum Payment {
    Pending,
    Settled { amount: u64, tx: String },
    Failed(String),
}

impl Payment {
    fn describe(&self) -> String {
        // `match self` in a &self method: bindings are references (amount: &u64, tx: &String)
        match self {
            Payment::Pending => "pending".to_string(),
            Payment::Settled { amount, tx } => format!("settled {amount} in {tx}"),
            Payment::Failed(reason) => format!("failed: {reason}"),
        }
    }

    fn amount(&self) -> Option<u64> {
        match self {
            Payment::Settled { amount, .. } => Some(*amount),
            Payment::Pending | Payment::Failed(_) => None,
        }
    }
}

// ---------- a state machine: data lives inside the state ----------
#[derive(Debug, PartialEq)]
enum Door {
    Open,
    Closed,
    Locked { code: u32 },
}

impl Door {
    // Returns false and changes nothing if the transition isn't allowed.
    fn lock(&mut self, code: u32) -> bool {
        match self {
            Door::Closed => {
                *self = Door::Locked { code }; // replace the whole value through &mut
                true
            }
            Door::Open | Door::Locked { .. } => false,
        }
    }

    fn unlock(&mut self, attempt: u32) -> bool {
        // `*self` is a place; `code` is Copy, so binding it copies the number out
        if let Door::Locked { code } = *self
            && code == attempt
        {
            *self = Door::Closed;
            return true;
        }
        false
    }
}

// ---------- Option ----------
fn price(item: &str) -> Option<u32> {
    match item {
        "apple" => Some(3),
        "bread" => Some(5),
        _ => None,
    }
}

// `?` returns None early if either price is missing.
fn basket(a: &str, b: &str) -> Option<u32> {
    let x = price(a)?;
    let y = price(b)?;
    Some(x + y)
}

fn main() {
    // enums with data
    let payments = [
        Payment::Pending,
        Payment::Settled { amount: 50, tx: String::from("0xab12") },
        Payment::Failed(String::from("insufficient gas")),
    ];
    for p in &payments {
        println!("{:<35} amount={:?}", p.describe(), p.amount());
    }

    // state machine
    let mut door = Door::Open;
    println!("lock while open: {} → {door:?}", door.lock(1234));
    door = Door::Closed;
    println!("lock while closed: {} → {door:?}", door.lock(1234));
    println!("unlock with 1111: {} → {door:?}", door.unlock(1111));
    println!("unlock with 1234: {} → {door:?}", door.unlock(1234));

    // Option basics
    let found = price("apple");
    let missing = price("caviar");
    println!("found={found:?} missing={missing:?}");
    println!("unwrap_or: {} {}", found.unwrap_or(0), missing.unwrap_or(0));
    println!("is_some: {} {}", found.is_some(), missing.is_some());
    // let n: u32 = found;              // ❌ E0308: Option<u32> is not u32, handle None first
    println!("basket(apple, bread)={:?}", basket("apple", "bread"));
    println!("basket(apple, caviar)={:?}", basket("apple", "caviar"));

    // looking inside an Option without moving it
    let nickname: Option<String> = Some(String::from("neo"));
    let as_str: Option<&str> = nickname.as_deref(); // borrow: Option<String> → Option<&str>
    println!("as_deref={as_str:?}, nickname still ours: {nickname:?}");

    // take(): move the value out, leave None behind
    let mut slot = Some(String::from("token"));
    let taken = slot.take();
    println!("taken={taken:?} slot={slot:?}");

    // memory layout: None is encoded as a null pointer, so the Option is free
    println!(
        "size_of: &u8={} Option<&u8>={} u32={} Option<u32>={}",
        std::mem::size_of::<&u8>(),
        std::mem::size_of::<Option<&u8>>(),
        std::mem::size_of::<u32>(),
        std::mem::size_of::<Option<u32>>(), // no spare bit pattern in u32 → needs a tag
    );
}
