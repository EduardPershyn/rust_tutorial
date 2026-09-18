# 03 — Structs, Enums & Pattern Matching

How you model data in Rust: no classes, no inheritance, no null. Instead: structs for "this AND that", enums for "this OR that", and `match` to take them apart safely.

```bash
cd 03_structs_enums
cargo run --example structs_methods   # also: enums_option, patterns
cargo test --test fix01_missing_variant
cargo test --lib
cargo test                            # everything
```

## Structs
```rust
#[derive(Debug, Clone, PartialEq)]       // auto-generate {:?}, .clone(), == (see derive below)
struct Account {
    owner: String,
    balance: u64,
}

let mut a = Account { owner: String::from("alice"), balance: 0 };
a.balance += 10;                          // `mut` applies to the whole value, not to single fields
let owner = String::from("bob");
let b = Account { owner, balance: 5 };    // shorthand: a field named like the variable
let c = Account { balance: 99, ..b };     // "the rest from b" (this MOVES b.owner into c)
```

| Kind | Declaration | Access |
|---|---|---|
| named fields | `struct Point { x: i32, y: i32 }` | `p.x` |
| tuple struct | `struct Wei(u128);` | `w.0` |
| unit struct | `struct Marker;` | (no data) |

**Newtype pattern:** `struct Wei(u128);` and `struct Gwei(u64);` compile to plain integers, zero cost, but the compiler won't let you pass a `Gwei` where `Wei` is expected. It prevents unit mix-ups like 6 vs 18 decimals.

## Methods: `impl`
```rust
impl Account {
    pub fn new(owner: &str) -> Self {                  // associated fn: no self, call as Account::new("x")
        Self { owner: owner.to_string(), balance: 0 }  // `Self` = the type being implemented
    }
    pub fn balance(&self) -> u64 { self.balance }      // read-only method
    pub fn deposit(&mut self, amount: u64) {           // modifying method
        self.balance += amount;
    }
    pub fn close(self) -> u64 { self.balance }         // consuming method: the Account is gone after
}

let mut acc = Account::new("alice");
acc.deposit(5);                  // auto-borrow: really Account::deposit(&mut acc, 5)
```

| Receiver | Meaning | C++ analogy |
|---|---|---|
| `&self` | read | `const` method |
| `&mut self` | modify | non-const method |
| `self` | take ownership: consume or transform | `&&`-qualified method |
| (none) | associated function | `static` method |

There are no constructors: `new` is only a naming convention. There's no inheritance either: shared behaviour comes from traits (Section 06).

## `derive`: auto-generated abilities
| Derive | Gives you | Requires |
|---|---|---|
| `Debug` | `{:?}` printing; needed by `assert_eq!` | all fields `Debug` |
| `Clone` | `.clone()` | all fields `Clone` |
| `Copy` | implicit copy instead of move | all fields `Copy` (no `String`, `Vec`, …) |
| `PartialEq`, `Eq` | `==`, `!=` | all fields comparable |
| `PartialOrd`, `Ord` | `<`, `>`, sorting | field order defines the ordering |
| `Hash` | usable as a `HashMap` key | `Eq` too |
| `Default` | `Type::default()` (0, "", empty, …) | all fields `Default` |

Rule of thumb: put `Debug` on almost everything, and `Copy` only on small plain data like `Point` or `Wei`.

## Enums: a value is ONE of several variants
```rust
enum Payment {
    Pending,                                // no data
    Settled { amount: u64, tx: String },    // struct-like variant
    Failed(String),                         // tuple-like variant
}
let p = Payment::Settled { amount: 5, tx: String::from("0xab") };
```
Each variant can carry **its own** data, which only exists while the value is that variant.

| Language | Equivalent |
|---|---|
| C++ | `std::variant` + `std::visit`, or a hand-made tagged union |
| Java 21 | `sealed interface` + `record`s + pattern-matching `switch` |
| Python | `Union[...]` + `isinstance`, checked only at runtime |
| Solidity | `enum` can't hold data at all |

**Make invalid states unrepresentable.** In Solidity you'd write `enum State { Created, Paid }` plus a separate `uint paidAmount`, which holds a meaningless value while the state is `Created`. In Rust, `Paid { amount }` puts the amount *inside* the state, so it can't exist in the wrong one.

## `Option<T>`: instead of null
```rust
enum Option<T> { Some(T), None }        // from std, always in scope
```
An `Option<u32>` is **not** a `u32`, so you can't use it before handling `None`. The compiler forces the check.

| You want | Write |
|---|---|
| the value or a fallback | `opt.unwrap_or(0)`, `opt.unwrap_or_default()` |
| just check | `opt.is_some()`, `opt.is_none()` |
| look inside without moving | `opt.as_ref()` (`&Option<T>` → `Option<&T>`), `opt.as_deref()` (`Option<String>` → `Option<&str>`) |
| take it out, leave `None` behind | `opt.take()` (like `mem::take`) |
| return `None` early | `let x = opt?;` inside a function that returns `Option` |
| crash if `None` | `opt.unwrap()`, `opt.expect("why it can't be None")` |

`unwrap()` is fine in tests, or when `None` would be a bug. Anywhere else it's a code smell, like this Section 01 pattern:
```rust
if r.is_none() { return 255; }   // ❌ check + unwrap: two steps that can drift apart
r.unwrap()

r.unwrap_or(255)                 // ✅ one step, can't panic
```

### `?` on Option
```rust
fn total(a: &str, b: &str) -> Option<u32> {
    let x = price(a)?;          // if price(a) is None → return None right here
    let y = price(b)?;
    Some(x + y)
}
```
This replaces a ladder of `match` / `if let`. The same `?` works with `Result` in Section 04.

## Pattern matching
`match` is exhaustive and is an expression (Section 01). Everything you can put on the left of `=>`:

| Pattern | Matches |
|---|---|
| `5`, `'a'`, `"text"` | a literal |
| `1 \| 2` | either one |
| `1..=9`, `1..10`, `..0`, `100..` | a range: inclusive, exclusive, or open-ended |
| `x` | anything, and binds it to `x` |
| `_` | anything, binds nothing |
| `n @ 1..=9` | a range, and binds the value to `n` |
| `(a, 0)` | a tuple, with parts |
| `Point { x, y: 0 }` | a struct: binds `x`, requires `y == 0` |
| `Point { x, .. }` | a struct, ignoring the other fields |
| `Some(x)`, `None`, `Payment::Failed(msg)` | an enum variant with its data |
| `[]`, `[x]`, `[first, .., last]`, `[head, rest @ ..]` | a slice, by length and shape |
| `pattern if cond` | a pattern plus an extra condition (a guard) |

Patterns nest: `Some(Payment::Settled { amount: 1..=100, .. })`.

### Guards break exhaustiveness checks
```rust
match n {                    // ❌ E0004: the compiler doesn't analyze `if` guards
    x if x < 0 => "neg",
    x if x >= 0 => "non-neg",
}
match n {                    // ✅ ranges are checked: this covers every i32, no `_` needed
    ..0 => "neg",
    0.. => "non-neg",
}
```
Use literals and ranges wherever you can. Keep guards for conditions a pattern can't express, like `(x, y) if x == y`.

### Avoid `_` on your own enums
Without `_`, adding a variant makes every `match` fail to compile until you handle the new case: a free to-do list. With `_`, the new variant silently lands in the default arm. Use `_` for numbers and strings, not for your own enums.

### Matching through a reference
```rust
let name: Option<String> = Some(String::from("neo"));

match &name {                // match on a REFERENCE...
    Some(n) => n.len(),      // ...and bindings become references too: n is &String
    None => 0,
};
println!("{name:?}");        // ✅ still usable: nothing was moved

match name {                 // match on the VALUE...
    Some(n) => n.len(),      // ...and `Some(n)` MOVES the String into n
    None => 0,
};
println!("{name:?}");        // ❌ E0382: name was (partially) moved
```
The same applies to `let`: `let Point { x, y } = &p;` borrows, `let Point { x, y } = p;` moves (or copies `Copy` fields).
Inside a `&self` method, `match self { … }` gives you references automatically.

## `if let`, `let else`, `while let`, `matches!`
```rust
if let Some(x) = opt { … }                     // care about one pattern only
if let Some(x) = opt { … } else { … }
if let Some(x) = opt && x > 10 { … }           // let chains (edition 2024)
let Some(x) = opt else { return 0; };          // bind or bail out: x is usable AFTER this line
while let Some(top) = stack.pop() { … }        // loop until the pattern stops matching
if matches!(p, Payment::Failed(_)) { … }       // a pattern as a bool
```

| Situation | Use |
|---|---|
| handle every case | `match` |
| act on one case, ignore the rest | `if let` |
| happy path continues, otherwise `return None` | `?` (clippy flags `let … else { return None }`) |
| happy path continues, otherwise exit differently (`continue`, `return false`, a default) | `let … else` |
| consume values until one doesn't fit | `while let` |

## Solidity analogy
| Solidity | Rust |
|---|---|
| `struct` + free functions | `struct` + `impl` block with methods |
| `enum State { A, B }` | `enum`, and variants can carry data |
| a missing mapping key reads as 0 | `Option<T>`: "missing" is explicit and must be handled |
| `require(state == State.Paid, "...")` | `match self { Order::Paid { amount } => …, _ => return false }` |

## Exercises
**Part A: make it compile** (`tests/fix*.rs`). Same rules as Section 02: fix it with the smallest change, and don't change what the asserts check. `fix07` compiles already: make its test pass.

**Part B: implement** (`src/ex*.rs`):
- `ex01_structs`: an `Account` with methods, plus `Gwei`/`Wei` newtypes
- `ex02_enums`: a traffic `Light`, and an `Order` state machine
- `ex03_patterns`: `Option`, `?`, range and slice patterns, `while let`, matching through references

Allowed: everything above, including `?` on `Option`.
Not yet: closures (`opt.map(|x| …)`, `unwrap_or_else(|| …)`), which come in Section 08.

The section is done when `cargo test` passes and `cargo clippy --all-targets` is clean.
