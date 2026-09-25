// Run: cargo run --example maps

use std::collections::{BTreeMap, HashMap};

fn main() {
    // ---------- basics ----------
    let mut scores: HashMap<String, u32> = HashMap::new();
    let previous = scores.insert("ann".to_string(), 10); // returns the OLD value
    scores.insert("bob".to_string(), 7);
    println!("insert returned {previous:?}, len={}", scores.len());

    println!("get(\"ann\")={:?}", scores.get("ann")); // Option<&u32>, &str searches a String key
    println!("get(\"eve\")={:?}", scores.get("eve"));
    println!("contains_key(\"bob\")={}", scores.contains_key("bob"));

    // get returns a REFERENCE: copy it out or unwrap it with a default
    let ann: u32 = scores.get("ann").copied().unwrap_or(0);
    let eve: u32 = scores.get("eve").copied().unwrap_or(0);
    println!("ann={ann} eve={eve}");

    // modifying through get_mut
    if let Some(score) = scores.get_mut("bob") {
        *score += 5;
    }
    println!("bob={:?}", scores.get("bob"));
    println!("removed ann: {:?}, len={}", scores.remove("ann"), scores.len());

    // ---------- the entry API ----------
    let text = "the quick brown fox jumps over the lazy dog the end";
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1; // insert 0 if missing, then add 1
    }
    println!("\"the\" appears {} times", counts["the"]);

    // building a map of Vec with or_default
    let mut by_letter: HashMap<char, Vec<&str>> = HashMap::new();
    for word in text.split_whitespace() {
        if let Some(first) = word.chars().next() {
            by_letter.entry(first).or_default().push(word);
        }
    }
    println!("words starting with 'o': {:?}", by_letter.get(&'o'));

    // ---------- iteration order is RANDOM ----------
    print!("HashMap order: ");
    for word in counts.keys() {
        print!("{word} ");
    }
    println!("\n  ↑ re-run this example: the order changes every time");

    // stable output: sort the pairs...
    let mut pairs: Vec<(&str, usize)> = counts.iter().map(|(w, c)| (*w, *c)).collect();
    pairs.sort_by_key(|(word, _)| *word);
    println!("sorted by word: {:?}", &pairs[..4]);

    // ...or use a BTreeMap, which is always sorted by key
    let sorted: BTreeMap<&str, usize> = counts.iter().map(|(w, c)| (*w, *c)).collect();
    print!("BTreeMap order: ");
    for (word, count) in &sorted {
        print!("{word}={count} ");
    }
    println!();
    println!("first key: {:?}, last key: {:?}", sorted.keys().next(), sorted.keys().last());

    // ---------- ownership of keys ----------
    let key = String::from("carol");
    let mut owned: HashMap<String, u32> = HashMap::new();
    owned.insert(key.clone(), 1); // clone: we want to keep using `key`
    owned.insert(key, 2); // this one MOVES the String into the map
    // println!("{key}");         // ❌ E0382: moved above
    println!("owned={owned:?}");

    // a tuple as a key: any type that is Eq + Hash works
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    grid.insert((0, 0), 'X');
    grid.insert((1, 2), 'O');
    println!("grid at (1,2): {:?}, at (5,5): {:?}", grid.get(&(1, 2)), grid.get(&(5, 5)));
}
