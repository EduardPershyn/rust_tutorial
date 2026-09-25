// Run: cargo run --example sets_strings

use std::collections::{BTreeSet, HashSet};

fn main() {
    // ---------- HashSet ----------
    let mut seen = HashSet::new();
    println!("insert(3) first time: {}", seen.insert(3)); // true = it was new
    println!("insert(3) again:      {}", seen.insert(3)); // false = already there
    seen.extend([1, 4, 1, 5]);
    println!("set={seen:?} len={} contains(&4)={}", seen.len(), seen.contains(&4));

    // finding duplicates with the return value of insert
    let values = [1, 2, 3, 2, 4, 1];
    let mut unique = HashSet::new();
    let mut duplicates = Vec::new();
    for v in values {
        if !unique.insert(v) {
            duplicates.push(v);
        }
    }
    println!("duplicates in {values:?}: {duplicates:?}");

    // ---------- set operations ----------
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let mut both: Vec<i32> = a.intersection(&b).copied().collect();
    both.sort(); // sets have no order, so sort for printing
    let mut only_a: Vec<i32> = a.difference(&b).copied().collect();
    only_a.sort();
    println!("intersection={both:?} difference={only_a:?} subset={}", a.is_subset(&b));

    // BTreeSet keeps its elements sorted
    let sorted: BTreeSet<&str> = ["pear", "apple", "fig", "apple"].into_iter().collect();
    println!("BTreeSet: {sorted:?}");

    // ---------- strings ----------
    let line = "  name = Ann Smith  ";
    println!("splitn(2, '='): {:?}", line.splitn(2, '=').collect::<Vec<_>>());
    println!("split(',') on \"a,b,,c\": {:?}", "a,b,,c".split(',').collect::<Vec<_>>());
    println!("rfind('/') in \"a/b/c\": {:?}", "a/b/c".rfind('/'));
    println!("replacen: {}", "a-b-c".replacen('-', "+", 1));
    println!("trim_matches('*'): {:?}", "**bold**".trim_matches('*'));
    println!("eq_ignore_ascii_case: {}", "HELLO".eq_ignore_ascii_case("hello"));

    // building text
    let parts = ["2026", "09", "25"];
    println!("join: {}", parts.join("-"));
    let mut out = String::with_capacity(32);
    out.push_str("total");
    out.insert(0, '>');
    out.push_str(": 42");
    println!("built: {out:?}");

    // char-aware work (never index a String)
    let word = "Привіт";
    let first_char_upper: String = word.chars().next().into_iter().collect();
    println!("{word}: {} chars, {} bytes, first={first_char_upper}", word.chars().count(), word.len());

    // literals
    let raw = r"C:\Users\file.txt"; // no escape processing
    let multi = "first
second";
    println!("raw={raw}  multi has {} lines", multi.lines().count());
    println!("bytes of \"abc\": {:?}", b"abc");
}
