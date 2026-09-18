//! FIX 04: a getter that tries to give a field away.
//! Make it compile WITHOUT `.clone()`. Don't change the test.
//! Question to answer for yourself: why does `decimals()` compile but `symbol()` doesn't?
//! Run: cargo test --test fix04_getter_moves

struct Token {
    symbol: String,
    decimals: u8,
}

impl Token {
    fn symbol(&self) -> String {
        self.symbol
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }
}

#[test]
fn getters() {
    let t = Token { symbol: String::from("USDC"), decimals: 6 };
    assert_eq!(t.symbol(), "USDC");
    assert_eq!(t.decimals(), 6);
    assert_eq!(t.symbol(), "USDC"); // still there after the first call
}
