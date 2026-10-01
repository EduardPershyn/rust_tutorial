//! Exercise 03: HashSet, BTreeSet and text formatting.
//! Run: cargo test --lib ex03

use std::collections::HashSet;

/// Does the slice contain the same value twice?
/// Hint: `insert` returns false when the value was already there.
pub fn has_duplicates(values: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &value in values {
        if !seen.insert(value) {
            return true;
        }
    }
    false
}

/// The set of distinct characters, ignoring whitespace and case (ASCII lowercase).
pub fn distinct_chars(s: &str) -> HashSet<char> {
    let mut chars = HashSet::new();
    for c in s.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }
        chars.insert(c.to_ascii_lowercase());
    }
    chars
}

/// Words that appear in BOTH texts (whitespace-separated, case-sensitive), sorted alphabetically.
pub fn common_words(a: &str, b: &str) -> Vec<String> {
    let a_words = a.split_whitespace().collect::<HashSet<&str>>();
    let b_words = b.split_whitespace().collect::<HashSet<&str>>();
    let mut common: Vec<String> = a_words.intersection(&b_words).map(|w| w.to_string()).collect();
    common.sort();
    common
}

/// Words of `text` that are not in `stop_words`, in their original order, without duplicates.
pub fn filter_stop_words(text: &str, stop_words: &HashSet<&str>) -> Vec<String> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for word in text.split_whitespace() {
        if !stop_words.contains(word) && seen.insert(word) {
            result.push(word.to_string());
        }
    }
    result
}

/// Capitalise the first character of every whitespace-separated word, lowercase the rest,
/// and join the words with single spaces. "  hELLO   big WORLD " → "Hello Big World"
/// Works with non-ASCII text too.
pub fn title_case(text: &str) -> String {
    let mut out = String::new();
    for word in text.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');                                // a space between words, not before the first
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());             // extend: push every char the iterator yields
            out.push_str(&chars.as_str().to_lowercase());
        }
    }
    out
}

/// Shorten to at most `max_chars` CHARACTERS. If it doesn't fit, cut it and append "…"
/// so that the result is exactly `max_chars` characters (the ellipsis counts as one).
/// truncate_chars("hello world", 8) == "hello w…"; shorter text is returned unchanged.
/// Returns "" when max_chars is 0.
pub fn truncate_chars(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if text.chars().count() <= max_chars {
        return text.to_string();                        // fits: unchanged, no "…"
    }
    let mut out: String = text.chars().take(max_chars - 1).collect();   // leave room for "…"
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_duplicates_values() {
        assert!(has_duplicates(&[1, 2, 3, 2]));
        assert!(!has_duplicates(&[1, 2, 3]));
        assert!(!has_duplicates(&[]));
    }

    #[test]
    fn distinct_chars_values() {
        let chars = distinct_chars("Hello World");
        assert_eq!(chars.len(), 7); // h e l o w r d
        assert!(chars.contains(&'h') && chars.contains(&'d'));
        assert!(!chars.contains(&'H') && !chars.contains(&' '));
        assert!(distinct_chars("  ").is_empty());
    }

    #[test]
    fn common_words_values() {
        assert_eq!(common_words("the quick fox", "the lazy fox dog"), ["fox", "the"]);
        assert_eq!(common_words("a b", "c d"), Vec::<String>::new());
        assert_eq!(common_words("The fox", "the fox"), ["fox"]); // case-sensitive
    }

    #[test]
    fn filter_stop_words_values() {
        let stop: HashSet<&str> = ["the", "a"].into_iter().collect();
        assert_eq!(filter_stop_words("the fox and a fox and the dog", &stop), ["fox", "and", "dog"]);
        assert_eq!(filter_stop_words("the a the", &stop), Vec::<String>::new());
    }

    #[test]
    fn title_case_values() {
        assert_eq!(title_case("  hELLO   big WORLD "), "Hello Big World");
        assert_eq!(title_case("rust"), "Rust");
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("привіт світ"), "Привіт Світ");
    }

    #[test]
    fn truncate_chars_values() {
        assert_eq!(truncate_chars("hello world", 8), "hello w…");
        assert_eq!(truncate_chars("hello", 8), "hello");
        assert_eq!(truncate_chars("hello", 5), "hello");
        assert_eq!(truncate_chars("hello", 4), "hel…");
        assert_eq!(truncate_chars("hello", 1), "…");
        assert_eq!(truncate_chars("hello", 0), "");
        assert_eq!(truncate_chars("привіт", 4), "при…"); // characters, not bytes
    }
}
