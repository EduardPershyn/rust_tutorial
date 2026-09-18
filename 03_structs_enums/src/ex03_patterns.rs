//! Exercise 03: Option, `?`, range and slice patterns, `while let`, matching through references.
//! Run: cargo test --lib ex03
//! Not allowed in this file: `unwrap()` and `expect()`.

/// Classify with range patterns ONLY: no `if` guards and no `_` arm.
/// < 0 → "negative", 0 → "zero", 1..=9 → "digit", 10..=99 → "two digits", ≥ 100 → "large".
pub fn describe(n: i32) -> &'static str {
    todo!()
}

/// (first, last) element; for a single element both are the same; None for an empty slice.
/// Use slice patterns (`[]`, `[x]`, `[first, .., last]`), not indexing / `.len()` / `.first()`.
pub fn first_and_last(v: &[i32]) -> Option<(i32, i32)> {
    todo!()
}

/// Sum of the values that are present. None only if BOTH are None.
/// Hint: match on the tuple `(a, b)`; an or-pattern can cover two cases in one arm.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

/// Pop items from the END of the stack, summing the numbers, until a `None` is popped
/// (that None is removed too) or the stack is empty. Items before the None stay on the stack.
/// Hint: `while let` with a nested pattern.
pub fn drain_sum(stack: &mut Vec<Option<i32>>) -> i32 {
    todo!()
}

/// The price of `item`, or None if it's not in the list.
pub fn lookup(prices: &[(&str, u32)], item: &str) -> Option<u32> {
    todo!()
}

/// Total price of all `items`. None if ANY item is missing from `prices`.
/// Hint: `lookup(...)?` inside the loop.
pub fn total(prices: &[(&str, u32)], items: &[&str]) -> Option<u32> {
    todo!()
}

/// Split "key = value" at the FIRST '=' and trim both sides.
/// None if there's no '=' or the key is empty. An empty value is fine.
/// Hint: `split_once` returns an Option; `?` gives you the parts or returns None for you.
pub fn parse_kv(line: &str) -> Option<(&str, &str)> {
    todo!()
}

pub struct User {
    pub name: String,
    pub nickname: Option<String>,
}

/// The nickname if the user has one, otherwise the name. Borrowed from `user`, no allocation.
pub fn display_name(user: &User) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRICES: [(&str, u32); 3] = [("apple", 3), ("bread", 5), ("milk", 2)];

    #[test]
    fn describe_values() {
        assert_eq!(describe(i32::MIN), "negative");
        assert_eq!(describe(-1), "negative");
        assert_eq!(describe(0), "zero");
        assert_eq!(describe(1), "digit");
        assert_eq!(describe(9), "digit");
        assert_eq!(describe(10), "two digits");
        assert_eq!(describe(99), "two digits");
        assert_eq!(describe(100), "large");
        assert_eq!(describe(i32::MAX), "large");
    }

    #[test]
    fn first_and_last_values() {
        assert_eq!(first_and_last(&[]), None);
        assert_eq!(first_and_last(&[7]), Some((7, 7)));
        assert_eq!(first_and_last(&[1, 2]), Some((1, 2)));
        assert_eq!(first_and_last(&[1, 2, 3, 4]), Some((1, 4)));
    }

    #[test]
    fn add_options_values() {
        assert_eq!(add_options(Some(2), Some(3)), Some(5));
        assert_eq!(add_options(Some(2), None), Some(2));
        assert_eq!(add_options(None, Some(3)), Some(3));
        assert_eq!(add_options(None, None), None);
    }

    #[test]
    fn drain_sum_stops_at_none() {
        let mut stack = vec![Some(1), None, Some(2), Some(3)];
        assert_eq!(drain_sum(&mut stack), 5);
        assert_eq!(stack, [Some(1)]); // the None was consumed, Some(1) stays
    }

    #[test]
    fn drain_sum_whole_stack() {
        let mut stack = vec![Some(4), Some(5)];
        assert_eq!(drain_sum(&mut stack), 9);
        assert!(stack.is_empty());

        let mut empty: Vec<Option<i32>> = Vec::new();
        assert_eq!(drain_sum(&mut empty), 0);

        let mut none_on_top = vec![Some(1), None];
        assert_eq!(drain_sum(&mut none_on_top), 0);
        assert_eq!(none_on_top, [Some(1)]);
    }

    #[test]
    fn lookup_values() {
        assert_eq!(lookup(&PRICES, "apple"), Some(3));
        assert_eq!(lookup(&PRICES, "milk"), Some(2));
        assert_eq!(lookup(&PRICES, "caviar"), None);
    }

    #[test]
    fn total_values() {
        assert_eq!(total(&PRICES, &["apple", "bread"]), Some(8));
        assert_eq!(total(&PRICES, &["milk", "milk", "milk"]), Some(6));
        assert_eq!(total(&PRICES, &["apple", "caviar", "bread"]), None);
        assert_eq!(total(&PRICES, &[]), Some(0));
    }

    #[test]
    fn parse_kv_values() {
        assert_eq!(parse_kv("name = Alice"), Some(("name", "Alice")));
        assert_eq!(parse_kv("x=1"), Some(("x", "1")));
        assert_eq!(parse_kv("a=b=c"), Some(("a", "b=c")));
        assert_eq!(parse_kv("key="), Some(("key", "")));
        assert_eq!(parse_kv("novalue"), None);
        assert_eq!(parse_kv("  = 5"), None);
    }

    #[test]
    fn display_name_values() {
        let neo = User { name: "Thomas Anderson".to_string(), nickname: Some("Neo".to_string()) };
        let ann = User { name: "Ann".to_string(), nickname: None };
        assert_eq!(display_name(&neo), "Neo");
        assert_eq!(display_name(&ann), "Ann");
        assert_eq!(display_name(&ann).as_ptr(), ann.name.as_ptr()); // borrowed, not copied
    }
}
