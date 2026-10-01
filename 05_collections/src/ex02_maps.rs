//! Exercise 02: HashMap and BTreeMap.
//! Run: cargo test --lib ex02
//! Remember: HashMap iteration order is random, so anything you RETURN must be made deterministic.

use std::collections::HashMap;
use std::cmp::Reverse;

/// Count how often each whitespace-separated word appears. Words own their text.
/// Hint: the `entry` API does this in one line per word.
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    counts
}

/// The most frequent word and its count. On a tie, the alphabetically smallest word.
/// None for an empty map.
pub fn most_common(counts: &HashMap<String, usize>) -> Option<(String, usize)> {
    let mut best: Option<(String, usize)> = None;
    for (word, &count) in counts {
        let is_better = match &best {                  // ← &best: look, don't take
            None => true,
            Some((best_word, best_count)) => {
                count > *best_count || (count == *best_count && word < best_word)
            }
        };
        if is_better {
            best = Some((word.clone(), count));        // clone only when there's a new leader
        }
    }
    best
}

/// Add every count from `extra` into `base` (same word → counts add up).
/// `extra` is consumed, so move its Strings instead of cloning them.
pub fn merge_counts(base: &mut HashMap<String, usize>, extra: HashMap<String, usize>) {
    // for (word, count) in extra {
    //     *base.entry(word).or_insert(0) += count; 
    // }

    for (word, count) in extra {
        match base.get_mut(&word) {
            Some(existing) => *existing += count,   // present: add in place
            None => {
                base.insert(word, count);           // absent: insert fresh
            }
        }
    }
}

/// Group the words by their first character. Words keep their order within a group.
/// Words that are empty are skipped.
pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut groups: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        let Some(first) = word.chars().next() else {
            continue;                                   // empty word: skip
        };
        groups.entry(first).or_default().push(word.to_string());
    }
    groups
}

/// Net balance per account from a list of (account, amount) movements.
/// Accounts whose net balance is exactly 0 must NOT appear in the result.
pub fn net_balances(movements: &[(&str, i64)]) -> HashMap<String, i64> {
    let mut result: HashMap<String, i64> = HashMap::new();
    for (account, amount) in movements {
        *result.entry(account.to_string()).or_default() += amount;
    }

    result.retain(|_, balance| *balance != 0);
    result
}

/// All (word, count) pairs, ordered by count descending, then by word ascending.
/// Hint: collect into a Vec first; `sort_by_key` with a tuple key sorts by the first part,
/// then the second. `std::cmp::Reverse(n)` flips the order of one part.
pub fn ranking(counts: &HashMap<String, usize>) -> Vec<(String, usize)> {
    // 1. copy the map's contents into a Vec: a Vec has an order, a HashMap doesn't
    let mut pairs: Vec<(String, usize)> = Vec::new();
    for (word, count) in counts {
        pairs.push((word.clone(), *count));
    }

    // 2. sort by a tuple key: first count (descending), then word (ascending)
    pairs.sort_by_key(|(word, count)| (Reverse(*count), word.clone()));
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counts_of(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for (word, count) in pairs {
            map.insert(word.to_string(), *count);
        }
        map
    }

    #[test]
    fn word_count_values() {
        let counts = word_count("the fox the dog  the");
        assert_eq!(counts.get("the"), Some(&3));
        assert_eq!(counts.get("fox"), Some(&1));
        assert_eq!(counts.get("cat"), None);
        assert_eq!(counts.len(), 3);
        assert!(word_count("   ").is_empty());
    }

    #[test]
    fn most_common_values() {
        assert_eq!(most_common(&counts_of(&[("a", 2), ("b", 5)])), Some(("b".to_string(), 5)));
        assert_eq!(most_common(&HashMap::new()), None);
    }

    #[test]
    fn most_common_breaks_ties_alphabetically() {
        let counts = counts_of(&[("pear", 4), ("apple", 4), ("fig", 4), ("kiwi", 1)]);
        // run it repeatedly: the answer must not depend on the random map order
        for _ in 0..20 {
            assert_eq!(most_common(&counts), Some(("apple".to_string(), 4)));
        }
    }

    #[test]
    fn merge_counts_values() {
        let mut base = counts_of(&[("a", 1), ("b", 2)]);
        merge_counts(&mut base, counts_of(&[("b", 3), ("c", 4)]));
        assert_eq!(base, counts_of(&[("a", 1), ("b", 5), ("c", 4)]));
    }

    #[test]
    fn group_by_first_char_values() {
        let groups = group_by_first_char(&["apple", "avocado", "banana", "", "art"]);
        assert_eq!(groups.get(&'a'), Some(&vec!["apple".to_string(), "avocado".to_string(), "art".to_string()]));
        assert_eq!(groups.get(&'b'), Some(&vec!["banana".to_string()]));
        assert_eq!(groups.len(), 2); // the empty word contributed nothing
    }

    #[test]
    fn net_balances_values() {
        let movements = [("ann", 100), ("bob", 50), ("ann", -30), ("carol", 20), ("carol", -20)];
        let balances = net_balances(&movements);
        assert_eq!(balances.get("ann"), Some(&70));
        assert_eq!(balances.get("bob"), Some(&50));
        assert_eq!(balances.get("carol"), None); // net zero: dropped
        assert_eq!(balances.len(), 2);
        assert!(net_balances(&[]).is_empty());
    }

    #[test]
    fn ranking_values() {
        let counts = counts_of(&[("the", 5), ("fox", 2), ("dog", 2), ("a", 9)]);
        assert_eq!(
            ranking(&counts),
            [
                ("a".to_string(), 9),
                ("the".to_string(), 5),
                ("dog".to_string(), 2), // same count as "fox" → alphabetical
                ("fox".to_string(), 2),
            ]
        );
        assert!(ranking(&HashMap::new()).is_empty());
    }
}
