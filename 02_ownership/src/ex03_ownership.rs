//! Exercise 03: moving ownership into and out of functions.
//! Run: cargo test --lib ex03
//! Rule: no `.clone()` in this file. Existing Strings must be MOVED, never copied.
//! Some tests compare heap addresses (`as_ptr()`) to check that nothing was copied.

/// Take ownership of `list`, append `item`, and give the list back.
/// Hint: a parameter can be `mut` (`fn f(mut list: Vec<String>, …)`); callers don't see the difference.
pub fn with_item(mut list: Vec<String>, item: &str) -> Vec<String> {
    list.push(item.to_string());
    list
}

/// Same effect, but borrowing instead of moving. This is the idiomatic version.
pub fn add_item(list: &mut Vec<String>, item: &str) {
    list.push(item.to_string());
}

/// Split into words that OWN their text, so they stay valid after `text` is gone.
pub fn split_owned(text: &str) -> Vec<String> {
    let mut result = Vec::new();

    for word in text.split_whitespace() {
        result.push(word.to_string());
    }
    result
}

/// Consume both lists and return one with all of `a` followed by all of `b`.
/// Move `b`'s Strings over; don't create new ones.
pub fn merge(mut a: Vec<String>, mut b: Vec<String>) -> Vec<String> {
    a.append(&mut b);
    a
}

/// Remove the longest string (in bytes) from the list and give it to the caller.
/// On a tie take the first one; None for an empty list. Keep the order of the remaining elements.
/// Hint: `list[i]` can't move a String out, but `list.remove(i)` returns it to you.
pub fn pop_longest(list: &mut Vec<String>) -> Option<String> {
    if list.is_empty() {
        return None;
    }

    let mut longest_index = 0;
    for i in 1..list.len() {
        if list[i].len() > list[longest_index].len() {
            longest_index = i;
        }
    }

    Some(list.remove(longest_index))
}

/// Take the String out of `slot`, leaving an empty String behind.
pub fn take_text(slot: &mut String) -> String {
    std::mem::take(slot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_item_round_trip() {
        let list = Vec::new();
        let list = with_item(list, "a"); // moved in, moved back out
        let list = with_item(list, "b");
        assert_eq!(list, ["a", "b"]);
    }

    #[test]
    fn add_item_borrows() {
        let mut list = vec![String::from("a")];
        add_item(&mut list, "b");
        add_item(&mut list, "c");
        assert_eq!(list, ["a", "b", "c"]);
    }

    #[test]
    fn split_owned_outlives_text() {
        let words;
        {
            let text = String::from("owned words survive");
            words = split_owned(&text);
        } // `text` is dropped here
        assert_eq!(words, ["owned", "words", "survive"]);
        assert!(split_owned("   ").is_empty());
    }

    #[test]
    fn merge_values() {
        let a = vec![String::from("a1")];
        let b = vec![String::from("b1"), String::from("b2")];
        assert_eq!(merge(a, b), ["a1", "b1", "b2"]);
        assert!(merge(Vec::new(), Vec::new()).is_empty());
    }

    #[test]
    fn merge_moves_not_copies() {
        let a = vec![String::from("a1")];
        let b = vec![String::from("b1"), String::from("b2")];
        let b1_heap = b[0].as_ptr();
        let merged = merge(a, b);
        assert_eq!(merged[1].as_ptr(), b1_heap); // same heap buffer: moved, not copied
    }

    #[test]
    fn pop_longest_values() {
        let mut list = vec![
            String::from("bb"),
            String::from("cccc"),
            String::from("dddd"),
            String::from("a"),
        ];
        let cccc_heap = list[1].as_ptr();

        let longest = pop_longest(&mut list).unwrap();
        assert_eq!(longest, "cccc");
        assert_eq!(longest.as_ptr(), cccc_heap); // moved out of the Vec, not copied
        assert_eq!(list, ["bb", "dddd", "a"]);

        assert_eq!(pop_longest(&mut list).unwrap(), "dddd");
        assert_eq!(pop_longest(&mut Vec::new()), None);
    }

    #[test]
    fn take_text_values() {
        let mut slot = String::from("payload");
        let heap = slot.as_ptr();
        let taken = take_text(&mut slot);
        assert_eq!(taken, "payload");
        assert_eq!(taken.as_ptr(), heap);
        assert_eq!(slot, "");
    }
}
