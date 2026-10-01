# Standard library cheat sheet

The std methods the exercises use, grouped by type. It grows with each section.
It isn't complete: it covers what you'll actually reach for. See "Finding methods yourself" at the end.

## `&str` (read-only text): also works on `String`
A `String` automatically gets every `&str` method, so look for string methods here first.

| Method | Returns | Example → result |
|---|---|---|
| `len()` | `usize` **bytes** | `"світ".len()` → `8` |
| `is_empty()` | `bool` | `"".is_empty()` → `true` |
| `chars()` | characters, one by one | `for c in s.chars()` |
| `chars().count()` | `usize` **characters** | `"світ".chars().count()` → `4` |
| `char_indices()` | `(byte_offset, char)` pairs | `for (i, c) in s.char_indices()` |
| `bytes()` | `u8` values | `for b in s.bytes()` |
| `split_whitespace()` | words as `&str` | `"a  b".split_whitespace()` → `"a"`, `"b"` |
| `split(',')` | pieces as `&str` | `"a,b".split(',')` → `"a"`, `"b"` |
| `lines()` | lines as `&str` | `"l1\nl2".lines()` → `"l1"`, `"l2"` |
| `split_once('=')` | `Option<(&str, &str)>` | `"a=b=c".split_once('=')` → `Some(("a", "b=c"))` |
| `find('l')` | `Option<usize>` byte index | `"hello".find('l')` → `Some(2)` |
| `contains("el")` | `bool` | |
| `starts_with("x")`, `ends_with("x")` | `bool` | |
| `strip_prefix("v")` | `Option<&str>` | `"v1.2".strip_prefix('v')` → `Some("1.2")` |
| `strip_suffix("%")` | `Option<&str>` | `"42%".strip_suffix('%')` → `Some("42")` |
| `trim()`, `trim_start()`, `trim_end()` | `&str` (a view, no copy) | `"  hi ".trim()` → `"hi"` |
| `to_uppercase()`, `to_lowercase()` | new `String` | |
| `replace("a", "b")` | new `String` | `"a-a".replace("a", "b")` → `"b-b"` |
| `repeat(3)` | new `String` | `"*".repeat(3)` → `"***"` |
| `parse::<i32>()` | `Result<i32, ParseIntError>` | `"42".parse::<i32>()` → `Ok(42)`; `"x"` → `Err(…)`. Or let the type drive it: `let n: u32 = s.parse()?;` |
| `to_string()` / `to_owned()` | new `String` | `&str` → `String` |
| `splitn(2, '=')` | at most n pieces | `"a=b=c".splitn(2, '=')` → `"a"`, `"b=c"` |
| `rsplit(' ')`, `rfind('/')` | from the right | `"a/b/c".rfind('/')` → `Some(3)` |
| `replacen('-', "+", 1)` | new `String` | replaces only the first n |
| `trim_matches('*')` | `&str` | `"**hi**".trim_matches('*')` → `"hi"` |
| `eq_ignore_ascii_case("x")` | `bool` | ASCII-only case-insensitive compare |

## `String` (owned, growable text): write operations
| Method | Effect |
|---|---|
| `String::new()`, `String::from("x")` | create |
| `push('c')` | append a char |
| `push_str("text")` | append a `&str` |
| `s += "text"` / `s + "text"` | append (`+` moves `s`) |
| `pop()` | remove the last char → `Option<char>` |
| `clear()` | make it empty, keeping the buffer |
| `as_str()` | `&str` view |
| `format!("{a}-{b}")` | build a new `String` from pieces |

## `char`
| Method | Returns |
|---|---|
| `is_alphabetic()`, `is_numeric()`, `is_alphanumeric()` | `bool` (Unicode-aware) |
| `is_whitespace()`, `is_ascii_digit()`, `is_ascii()` | `bool` |
| `to_ascii_lowercase()`, `to_ascii_uppercase()` | `char` (ASCII only) |
| `to_lowercase()`, `to_uppercase()` | an iterator of chars (loop over it; some letters become 2 chars) |
| `to_digit(10)` | `Option<u32>`: `'7'.to_digit(10)` → `Some(7)` |

## Integers
| Method | Returns / note |
|---|---|
| `checked_add(x)` (also `_sub`, `_mul`, `_div`) | `Option`: `None` on overflow |
| `saturating_add(x)` | clamps at MIN/MAX |
| `wrapping_add(x)` | wraps around |
| `abs()`, `unsigned_abs()` | `unsigned_abs` is safe for `MIN` |
| `abs_diff(y)` | distance, never overflows |
| `pow(3)`, `min(y)`, `max(y)` | |
| `rem_euclid(n)` | always non-negative modulo |
| `is_multiple_of(n)` | `bool`, **unsigned types only** (for signed: `x % n == 0`) |
| `i32::MIN`, `u64::MAX` | limits |

## Slices `&[T]` and `Vec<T>`
Like `String`/`&str`: a `Vec` gets every slice method.

| Method | Returns / effect |
|---|---|
| `len()`, `is_empty()` | |
| `first()`, `last()` | `Option<&T>` |
| `get(i)` | `Option<&T>`, never panics (`v[i]` panics) |
| `contains(&x)` | `bool` (takes a reference) |
| `iter()`, `iter_mut()`, `into_iter()` | `&T` / `&mut T` / owned `T` items (the last consumes the collection); `.enumerate()` adds indexes, `.rev()` reverses |
| `swap(i, j)`, `reverse()` | in place |
| `sort()`, `sort_unstable()` | in place |
| `split_at(mid)` | `(&[T], &[T])` |
| **Vec only:** `push(x)`, `pop()` → `Option<T>` | add or remove at the end |
| `insert(i, x)`, `remove(i)` → `T` | shifts the elements after it |
| `swap_remove(i)` → `T` | O(1), but changes the order |
| `extend(other)`, `append(&mut other)` | add many |
| `clear()`, `truncate(n)` | |

## `Vec<T>` extras (Section 05)
| Method | Effect |
|---|---|
| `Vec::with_capacity(n)`, `capacity()` | pre-allocate room for n items / how many fit before regrowing |
| `to_vec()` on a slice | copy a `&[T]` into a new `Vec<T>` |
| `dedup()` | remove **consecutive** duplicates (sort first) |
| `sort_by_key(\|x\| …)` | sort by a computed key; `std::cmp::Reverse(k)` flips the order |
| `sort_by(\|a, b\| a.partial_cmp(b).unwrap())` | needed for floats |
| `retain(\|x\| …)` | keep only the elements matching the condition |
| `binary_search(&x)` | `Result<found_index, insert_index>`, on a sorted Vec |
| `windows(n)` | every overlapping run of n elements |
| `chunks(n)` | non-overlapping groups of n (the last may be shorter) |
| `join(", ")` | `[String]` / `[&str]` → `String`; `concat()` flattens `[Vec<T>]` |
| `.iter().map(…).collect()` | build a new collection (Section 08) |

## `HashMap<K, V>` / `BTreeMap<K, V>` (Section 05)
`use std::collections::HashMap;`

| Method | Returns / effect |
|---|---|
| `HashMap::new()`, `HashMap::from([(k, v), …])` | create |
| `insert(k, v)` | `Option<V>`: the previous value. **Moves** k and v into the map |
| `get(&k)`, `get_mut(&k)` | `Option<&V>` / `Option<&mut V>`; a `String` key accepts `get("text")` |
| `contains_key(&k)`, `remove(&k)` | `bool` / `Option<V>` |
| `map[&k]` | the value, **panics** if absent |
| `entry(k).or_insert(0)` | `&mut V`: insert if missing, then give access → `*…or_insert(0) += 1` |
| `entry(k).or_default()` | the same with the type's default (`0`, `""`, empty `Vec`) |
| `keys()`, `values()`, `values_mut()`, `iter()` | iterators, in **random** order for `HashMap` |
| `len()`, `is_empty()`, `retain(\|k, v\| …)` | |

`BTreeMap` has the same API, iterates **sorted by key**, and adds `range(a..b)`. Keys need `Ord` instead of `Hash`.

## `HashSet<T>` / `BTreeSet<T>` (Section 05)
| Method | Returns |
|---|---|
| `insert(x)` | `bool`: **true if it was new** (handy for duplicate detection) |
| `contains(&x)`, `remove(&x)` | `bool` |
| `intersection(&b)`, `union(&b)`, `difference(&b)` | iterators (`.collect()` them, then sort for stable output) |
| `is_subset(&b)`, `is_disjoint(&b)` | `bool` |

## `Option<T>`
| Method | Returns |
|---|---|
| `is_some()`, `is_none()` | `bool` |
| `unwrap_or(x)`, `unwrap_or_default()` | `T`, never panics |
| `unwrap()`, `expect("msg")` | `T`, **panics** on `None` |
| `take()` | `Option<T>`, leaves `None` behind |
| `replace(x)` | the old `Option<T>`, puts `Some(x)` in |
| `as_ref()` | `&Option<T>` → `Option<&T>` |
| `as_deref()` | `Option<String>` → `Option<&str>` |
| `copied()` | `Option<&i32>` → `Option<i32>` |
| `ok_or(err)` | `Result<T, E>`: `Some(v)` → `Ok(v)`, `None` → `Err(err)` |
| `opt?` | the value, or return `None` from the current function |

## `Result<T, E>` (Section 04)
| Method | Returns |
|---|---|
| `is_ok()`, `is_err()` | `bool` |
| `unwrap_or(x)`, `unwrap_or_default()` | `T`, never panics |
| `ok()` | `Option<T>`: forgets the error |
| `map_err(MyError::Variant)` | `Result<T, MyError>`: wraps the error in your variant |
| `r?` | the value, or return `Err(From::from(e))` from the current function |
| `unwrap()`, `expect("msg")`, `unwrap_err()` | panic on the wrong variant: tests only |
| `.to_string()` on an error | its `Display` message |

## Formatting into a `Formatter` (for `impl Display`)
| Macro | Effect |
|---|---|
| `write!(f, "need {x}")` | like `format!`, but writes into `f`; returns `fmt::Result` |

## `std::mem`
| Function | Effect |
|---|---|
| `mem::take(&mut x)` | move out, leave the default (`""`, empty, `0`, `None`) |
| `mem::replace(&mut x, new)` | move out, put `new` in |
| `mem::swap(&mut a, &mut b)` | exchange |

## Finding methods yourself
The std library is huge, and Rust developers look things up constantly. It's a normal part of writing Rust, not a lack of knowledge.
- **Autocomplete:** type `line.` in Cursor and scroll the list; each entry shows its signature. Hover a method to read its docs.
- **Offline docs:** `rustup doc --std`, then search (e.g. `split_once`). Each type's page has a "Methods" section.
- **Search by what you need:** on doc.rust-lang.org/std, search `str` and skim the method names. They're descriptive (`strip_prefix`, `split_once`, `trim_end`).
- **Look in the right place:** `String` methods are mostly listed under `str`, `Vec` methods under `slice`, and `Option` has its own page.
