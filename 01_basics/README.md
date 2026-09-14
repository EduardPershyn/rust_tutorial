# 01 — Basics

```bash
cd 01_basics
cargo run --example variables_types   # also: functions, control_flow
cargo test                            # exercises
```

## Cargo in 30 seconds
| Command | What |
|---|---|
| `cargo new app` / `cargo new --lib mylib` | new project (binary / library) |
| `cargo check` | type-check only (fast, use it constantly) |
| `cargo run` | build (debug) + run |
| `cargo build --release` | optimized build |
| `cargo test` | run tests (`cargo test foo` runs only tests whose name contains `foo`) |
| `cargo clippy` / `cargo fmt` | lint / format |

`Cargo.toml` plays the role of `package.json` / `pom.xml`. `src/main.rs` is the binary entry point, `src/lib.rs` the library root.

## Variables
```rust
let x = 5;               // IMMUTABLE by default
let mut y = 5;           // opt-in mutability
y += 1;                  // no ++ / -- in Rust
let x = x * 2;           // shadowing: a NEW variable with the same name (type may change)
let z: i64 = 10;         // explicit type (otherwise inferred)
const MAX: u32 = 1_000;  // compile-time constant, type required, SCREAMING_CASE
```

## Scalar types
| Rust | Analog |
|---|---|
| `i8 i16 i32 i64 i128` | `int8_t` … / Solidity `int8` … |
| `u8 u16 u32 u64 u128` | unsigned |
| `isize` / `usize` | pointer-sized; **`usize` is used for indexes and lengths** |
| `f32` / `f64` | `float` / `double` |
| `bool` | `bool` |
| `char` | 4-byte Unicode scalar value, **not** a byte (a byte is `u8`) |

Defaults: integer literal → `i32`, float literal → `f64`.

**No implicit conversions**, not even `i32 → i64`:
```rust
let a: i32 = 300;
let b = a as i64;               // cast with `as` (can silently truncate: 300 as u8 == 44)
let c: i64 = a.into();          // lossless conversion, always succeeds
let d = u8::try_from(a);        // checked conversion → Err here (Result, section 04)
```

**Overflow**: in a debug build it panics, in release it wraps. To choose explicitly (like Solidity 0.8 `unchecked`):
```rust
250u8.checked_add(10)     // None
250u8.wrapping_add(10)    // 4
250u8.saturating_add(10)  // 255
```

## Tuples & arrays
```rust
let t: (i32, f64, char) = (1, 2.0, 'a');
let (a, b, c) = t;          // destructuring
let first = t.0;            // access by index

let arr: [i32; 3] = [1, 2, 3];   // fixed size, the size is part of the type, lives on the stack
let zeros = [0u8; 100];          // 100 zeros
let grid = [[0; 3]; 3];          // 2D
arr.len();
arr[10];                         // out of bounds → panic (checked, never UB)
```
The growable array is `Vec<T>` (section 05).

## Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b              // NO semicolon → this is the return value
}

fn log(msg: &str) {    // no "-> T" → returns () ("unit", like void)
    println!("{msg}");
}
```
- Parameter and return types are always explicit (no inference in signatures).
- `return x;` is only used for early returns.
- **Almost everything is an expression.** A trailing `;` turns it into a statement of type `()`.
  The error `expected i32, found ()` usually means you added a `;` you didn't want.

```rust
let y = { let x = 3; x + 1 };   // a block is an expression → y == 4
```

## Control flow
```rust
let kind = if n % 2 == 0 { "even" } else { "odd" };  // if is an expression, so there's no ?: ternary
// condition must be bool: `if n { }` does not compile for integers. No parens, braces required.

loop { /* forever */ }
let v = loop { break 42; };           // loop can return a value
while n > 0 { n -= 1; }

for i in 0..5 {}                      // 0,1,2,3,4
for i in 0..=5 {}                     // inclusive
for i in (0..5).rev() {}              // 4..0
for x in arr {}                       // over array elements
for (i, x) in arr.into_iter().enumerate() {}   // with index

'outer: for i in 0..10 {              // labels for nested loops
    for j in 0..10 {
        if i * j > 20 { break 'outer; }       // also: continue 'outer;
    }
}
```

`match` is `switch` on steroids. It must be **exhaustive** and it returns a value:
```rust
let s = match n {
    0 => "zero",
    1 | 2 => "small",          // or-pattern
    3..=9 => "medium",         // range
    x if x < 0 => "negative",  // binding + guard
    _ => "large",              // _ = anything else
};
```

## Option (preview)
Rust has no null. A value that may be missing is `Option<T>`, which is either `Some(value)` or `None`.
```rust
fn find(arr: [i32; 3], x: i32) -> Option<usize> {
    for (i, v) in arr.into_iter().enumerate() {
        if v == x { return Some(i); }
    }
    None
}

match find([1, 2, 3], 2) {
    Some(i) => println!("found at {i}"),
    None => println!("not found"),
}
```
The full story is in section 03.

## Printing & macros
`name!(...)` is a **macro**: code generated at compile time, and it can take a variable number of arguments.
```rust
println!("{} + {} = {}", a, b, a + b);
println!("{a} {b}");         // inline names (plain variables only, not expressions)
println!("{:?}", (1, 2));    // Debug format: tuples, arrays, most std types
println!("{arr:#?}");        // pretty Debug
println!("{:.2}", 1.23456);  // 1.23
let s = format!("x={x}");    // same syntax, returns String
dbg!(x * 2);                 // prints [file:line] x * 2 = 10, returns the value
```

## Strings (bare minimum for now)
- `"hello"` has type `&str`, a borrowed string slice. Literals are `&'static str` (they live for the whole program).
- `String` is an owned, growable, heap-allocated string.
- To get a `String`: `"hi".to_string()`, `String::from("hi")`, `42.to_string()`, `format!(...)`.

Why two types: that's ownership (section 02).

## Tests
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]          // compiled only by `cargo test`
mod tests {
    use super::*;     // import everything from the parent module
    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
        assert!(add(1, 1) > 0, "optional message {}", 42);
    }
}
```
`todo!()` is a placeholder: it compiles for any return type and panics at runtime.

## Gotchas coming from C++ / Java / Python
- Immutable by default. Needing `mut` everywhere is a design smell.
- No implicit numeric conversions, no `++`, no ternary `?:`.
- Integer `/` truncates toward zero and `%` can be negative (`-7 % 3 == -1`). For math modulo use `rem_euclid`.
- `x.abs()` on `i32::MIN` overflows → panic. Use `unsigned_abs()`.
- Out-of-bounds indexing and overflow panic instead of causing UB.
- Unused variables and `mut` produce warnings. Prefix a name with `_` to silence one on purpose.

## Exercises
Files in `src/`: `ex01_types_functions.rs`, `ex02_control_flow.rs`, `ex03_arrays_tuples.rs`.
```bash
cargo test            # everything
cargo test ex01       # a single file
cargo clippy          # then make it idiomatic
```
Replace every `todo!()`. Don't use iterator methods like `.iter().sum()` / `.min()` yet: practice loops first.
