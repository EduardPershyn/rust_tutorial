# 05 — Collections & Strings

The containers you'll use every day, and the text handling that goes with them. After this section, the hand-written `Vec<(String, u64)>` lookups from Sections 03–04 become one-liners.

```bash
cd 05_collections
cargo run --example vectors          # also: maps, sets_strings
cargo test --test fix01_option_ref
cargo test --lib
cargo test                           # everything
```

## Choosing a collection
| Need | Type | Lookup | Notes |
|---|---|---|---|
| a list, in order | `Vec<T>` | by index O(1), by value O(n) | the default choice |
| key → value | `HashMap<K, V>` | O(1) | iteration order is **random** |
| key → value, sorted | `BTreeMap<K, V>` | O(log n) | iterates in key order; supports ranges |
| "have I seen this?" | `HashSet<T>` | O(1) | a map with no values |
| the same, sorted | `BTreeSet<T>` | O(log n) | |
| push/pop at **both** ends | `VecDeque<T>` | O(1) at both ends | a queue |

Rule of thumb: start with `Vec`, switch to `HashMap` when you find yourself searching by key, and to `BTreeMap` when you need sorted output or range queries.

| Language | Map | Set |
|---|---|---|
| C++ | `unordered_map` / `map` | `unordered_set` / `set` |
| Java | `HashMap` / `TreeMap` | `HashSet` / `TreeSet` |
| Python | `dict` (insertion-ordered) | `set` |
| Solidity | `mapping` (no iteration, no length) | — |

Unlike a Solidity `mapping`, a Rust map knows its length, can be iterated, and tells "absent" apart from "zero".

## Two things you need from Section 08, early
Collection methods often take a small inline function, and gather results back into a collection. The full story is Section 08; the short version:

```rust
|x| x * 2                  // a closure: an anonymous function. `x` is the parameter, no types needed
|&(name, _)| name          // the parameter can be a pattern, as in a `for` loop

let v: Vec<u32> = map.keys().copied().collect();   // collect(): iterator → a collection
```
`collect()` needs to know the target type, from the annotation (`let v: Vec<_> = …`) or a turbofish (`collect::<Vec<_>>()`).

### Iterators in one table
An iterator hands out items one at a time (a `for` loop drives one). Three ways to get one from a collection:

| Call | Items | The collection afterwards | Same as |
|---|---|---|---|
| `v.iter()` | `&T`, borrowed | still usable | `for x in &v` |
| `v.iter_mut()` | `&mut T`, writable | still usable | `for x in &mut v` |
| `v.into_iter()` | `T`, **moved out** | **consumed** | `for x in v` |

Maps have their own: `map.iter()` gives `(&K, &V)` pairs, plus `keys()`, `values()`, `values_mut()`. Strings have `chars()` and `split_whitespace()`.

What you can chain on an iterator in this section:
```rust
.map(|x| x * 2)          // transform each item
.copied()  .cloned()     // &T → T (Copy types) / &T → T via clone (e.g. &String → String)
.rev()  .enumerate()     // backwards / with indexes (i, item)
.collect()               // gather into a Vec, String, HashMap, HashSet…: the type annotation decides
```
Iterators are lazy: nothing runs until something consumes them (`for`, `collect`, `sum`…). The rest (`filter`, `fold`, `zip`, writing your own) is Section 08.

## `Vec<T>`
```rust
let mut v = vec![3, 1, 2];               // or Vec::new(), Vec::with_capacity(100)
v.push(4);                                // add at the end
v.pop();                                  // Option<T> from the end
v.insert(0, 9);                           // shifts everything right: O(n)
v.remove(0);                              // shifts everything left: O(n), returns the element
v.swap_remove(0);                         // O(1), but moves the LAST element into the hole
v.extend([5, 6]);                         // append many
v.truncate(2); v.clear();
v.contains(&3);                           // O(n)
v[0];  v.get(99);                         // panics  /  Option<&T>
v.first(); v.last();                      // Option<&T>
v.sort(); v.sort_unstable();              // in place; unstable is faster, ties may be reordered
v.sort_by_key(|x| -x);                    // sort by a computed key
v.dedup();                                // removes CONSECUTIVE duplicates → sort first
v.reverse(); v.swap(0, 1);
v.retain(|x| *x != 0);                    // keep the elements matching the condition
v.binary_search(&3);                      // Result<index_found, index_where_it_would_go>; needs a sorted Vec
v.windows(3);                             // every overlapping run of 3: [0,1,2], [1,2,3], …
v.chunks(3);                              // non-overlapping groups of 3 (the last may be shorter)
v.concat(); v.join(", ");                 // Vec<Vec<T>> → Vec<T>; Vec<String> → String
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

**Capacity:** a `Vec` owns a heap buffer with room for `capacity()` items; pushing past it allocates a bigger one and moves everything. `Vec::with_capacity(n)` avoids the repeated growth when you know the size, and it's why holding a reference into a `Vec` blocks `push` (Section 02).

Sorting floats needs `sort_by(|a, b| a.partial_cmp(b).unwrap())`, because `f64` has `NaN` and therefore no total order.

**Descending order with `std::cmp::Reverse`.** `Reverse` is a newtype, `struct Reverse<T>(pub T)`, whose comparison is the opposite of the wrapped value's: `Reverse(5) < Reverse(3)` is true. Wrap a key to flip its order:
```rust
use std::cmp::Reverse;
v.sort_by_key(|x| Reverse(*x));                             // largest first
pairs.sort_by_key(|(word, n)| (Reverse(*n), word.clone())); // count descending, then word ascending
```
Tuple keys compare field by field, so wrapping only one part flips only that part, which `v.sort(); v.reverse()` can't do. (In `sort_by_key` the key can't borrow from the element, hence `word.clone()`; `sort_by(|a, b| …)` avoids the clone by comparing directly.)

## `HashMap<K, V>`
```rust
use std::collections::HashMap;

let mut scores: HashMap<String, u32> = HashMap::new();
scores.insert("ann".to_string(), 10);     // returns the OLD value as Option<V>
scores.get("ann");                         // Option<&u32> — note: &str works for a String key
scores.get_mut("ann");                     // Option<&mut u32>
scores.contains_key("ann");
scores.remove("ann");                      // Option<V>: gives you the value back
scores.len(); scores.is_empty();
scores["ann"];                             // panics if absent — prefer get()

for (name, score) in &scores { }           // &K, &V, in RANDOM order
for (name, score) in &mut scores { }       // &K, &mut V
for (name, score) in scores { }            // consumes the map: K, V by value
scores.keys(); scores.values(); scores.values_mut();
```

### The `entry` API: "insert if missing, then update"
```rust
*counts.entry(word).or_insert(0) += 1;         // the classic word counter
counts.entry(word).or_default();                // 0, "", empty Vec…
groups.entry(letter).or_default().push(name);   // build a map of Vec
map.entry(k).or_insert_with(expensive_call);    // only calls it when the key is missing
```
`entry` looks the key up **once**. Written by hand it would be a `contains_key` + `get_mut` + `insert` dance, with two or three lookups.

### Keys: ownership and requirements
- Inserting **moves** the key and value into the map: `map.insert(name, 5)` consumes `name`. Use `name.clone()` if you still need it.
- Looking up **borrows**: a `HashMap<String, V>` accepts `get("ann")` with a `&str`, so no allocation is needed to search.
- A key type must implement `Eq + Hash`: integers, `String`, `&str`, `char`, tuples of those, and your own types with `#[derive(PartialEq, Eq, Hash)]`. `f64` can't be a key (`NaN`).

### Iteration order is random
`HashMap` (and `HashSet`) iterate in an unpredictable order, which even changes between runs. Never assert on it in a test. To get stable output, either sort the collected pairs or use a `BTreeMap`.

## `HashSet<T>`, `BTreeMap`, `BTreeSet`
```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
seen.insert(3);                 // returns true if it was NEW (false = already there)
seen.contains(&3);              // note the &
seen.remove(&3);
a.intersection(&b);             // in both: an iterator, so loop over it or .collect() it
a.union(&b);                    // in either
a.difference(&b);               // in a but not in b
a.symmetric_difference(&b);     // in exactly one of them
a.is_subset(&b); a.is_disjoint(&b);
let both: HashSet<_> = &a & &b; // the operators &, |, -, ^ do the same and return a new set
```
`BTreeMap` / `BTreeSet` have the same API plus ordering: iteration is sorted by key, and `map.range("a".."m")` gives a slice of the key space. Keys need `Ord` instead of `Hash`.

## Strings, the rest of the story
Section 02 covered `String` vs `&str`, UTF-8, and slicing. What's left:
```rust
let mut s = String::with_capacity(64);
s.insert(0, 'x'); s.insert_str(0, "ab");   // O(n): shifts the rest
s.remove(0);                                // returns the char
s.truncate(3); s.clear();

"a,b,,c".split(',');                        // 4 pieces, including the empty one
"a,b,c".splitn(2, ',');                     // at most 2 pieces: "a", "b,c"
"a b".rsplit(' ');                          // from the right
"  x  ".trim_matches('x');                  // trims the given char from both ends
"path/to/file".rfind('/');                  // last position
"a-b-c".replacen('-', "+", 1);              // replace only the first n
["a", "b"].join("-");                       // "a-b"
"HELLO".eq_ignore_ascii_case("hello");      // true
"ß".to_uppercase();                          // "SS": one char can become two

let raw = r"C:\dir\file";                   // raw string: no escapes
let multi = "line1
line2";                                      // literals can span lines
let bytes = b"abc";                          // &[u8; 3], byte string
```
Careful: `to_uppercase()` allocates and is Unicode-aware; `to_ascii_uppercase()` is cheaper and only touches a–z.

## Complexity cheat sheet
| Operation | `Vec` | `HashMap` | `BTreeMap` |
|---|---|---|---|
| push / insert | O(1)* | O(1)* | O(log n) |
| lookup by key | O(n) | O(1) | O(log n) |
| lookup by index | O(1) | — | — |
| remove from the middle | O(n) | O(1) | O(log n) |
| sorted iteration | sort first: O(n log n) | sort first | free |

\* amortised: occasionally a reallocation copies everything.

## Exercises
**Part A: make it compile / pass** (`tests/fix*.rs`), same rules as before. Some already compile: make their tests pass.

**Part B: implement** (`src/ex*.rs`):
- `ex01_vectors`: sorting, dedup, windows, chunks, join
- `ex02_maps`: word counting with `entry`, merging maps, grouping, deterministic ranking
- `ex03_sets_strings`: `HashSet` for duplicates and intersections, plus text formatting

Allowed now: simple closures (`|x| …`) for `sort_by_key` / `retain`, and `.collect()`.
Not yet: longer iterator chains (`filter`, `fold`, `zip`…), which are Section 08.

New std methods are in [STD_CHEATSHEET.md](../STD_CHEATSHEET.md) under Vec, HashMap, HashSet and `&str`.

The section is done when `cargo test` passes and `cargo clippy --all-targets` is clean.
