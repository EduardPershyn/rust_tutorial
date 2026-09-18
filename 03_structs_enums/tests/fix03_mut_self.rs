//! FIX 03: a method that modifies its struct.
//! Make it compile. Don't change the asserts. (There are two places to fix; the second error
//! only shows up after you fix the first.)
//! Run: cargo test --test fix03_mut_self

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&self) {
        self.count += 1;
    }

    fn get(&self) -> u32 {
        self.count
    }
}

#[test]
fn counts() {
    let c = Counter::new();
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}
