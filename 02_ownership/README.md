# 02 — Ownership & Borrowing

This is the concept that makes Rust different. Everything later in the language builds on it.

```bash
cd 02_ownership
cargo run --example moves              # also: borrowing, slices_strings
cargo test --test fix01_use_after_move # Part A: make-it-compile exercises (tests/)
cargo test --lib                       # Part B: implement exercises (src/)
cargo test                             # everything: the section is done when this is green
```

## Stack vs heap
```
let s = String::from("hello");

  stack (s)            heap
 ┌─────────┐          ┌───┬───┬───┬───┬───┐
 │ ptr ────┼────────► │ h │ e │ l │ l │ o │
 │ len = 5 │          └───┴───┴───┴───┴───┘
 │ cap = 5 │
 └─────────┘
```
`String`, `Vec<T>` and `Box<T>` own heap memory. `i32`, `f64`, `bool`, `char`, and arrays or tuples of these live entirely on the stack.

## The 3 ownership rules
1. Every value has exactly **one owner**: a variable, a field, a slot in a `Vec`, …
2. When the owner goes out of scope, the value is **dropped** (its memory is freed). This is deterministic, like a C++ destructor (RAII), with no GC.
3. Ownership can be **moved** to a new owner, and the old one becomes unusable.

## Move
```rust
let a = String::from("hi");
let b = a;                  // MOVE: copies (ptr, len, cap), the heap is untouched, `a` is dead
println!("{a}");            // ❌ E0382: borrow of moved value `a`

fn consume(s: String) {}    // a by-value parameter = move into the function
consume(b);                 // b is gone; the String is dropped when consume() ends

fn make() -> String { String::from("x") }   // returning = move out to the caller (no copy)
```

| | C++ | Rust |
|---|---|---|
| `b = a` | deep copy | **move** (shallow, `a` is dead) |
| explicit copy | — | `let b = a.clone();` |
| explicit move | `std::move(a)` | — (it's the default) |
| use after move | compiles, object in unspecified state | compile error |

**`Copy` types are the exception**: they are copied instead of moved. These are integers, floats, `bool`, `char`, shared references `&T`, and tuples or arrays made only of `Copy` types. Everything in Section 01 was `Copy`, which is why you never hit a move error there.

`.clone()` makes an explicit deep copy and allocates. It's fine when you really need a copy, but cloning just to silence the borrow checker is a code smell.

## Borrowing (references)
Instead of moving a value, you can lend access to it:
```rust
fn count(s: &String) -> usize { s.len() }      // shared borrow: read only (`&str` is even better, see Slices)
fn shout(s: &mut String) { s.push('!'); }      // mutable borrow: can modify

let mut s = String::from("hi");
let n = count(&s);       // & creates a shared reference
shout(&mut s);           // &mut creates a mutable reference (s must be `let mut`)
```

### The one rule
At any moment a value can have **either**:
- any number of **shared** references `&T`, **or**
- exactly **one mutable** reference `&mut T`.

On top of that, a reference can never outlive the value it points to.

Think of it as a read-write lock that the compiler checks, with zero runtime cost. It rejects code that would be undefined behaviour in C++:
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];       // shared borrow of v
v.push(4);               // ❌ E0502: push needs &mut v while `first` is alive
println!("{first}");     //    (push may reallocate, which would leave `first` dangling)
```

### A borrow ends at its last use
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
println!("{first}");     // last use of `first`: the borrow ends here
v.push(4);               // ✅ OK
```
Often the fix for a borrow error is just **reordering** statements.

### Dereferencing
```rust
let mut x = 10;
let r = &mut x;
*r += 1;                 // `*` reaches the value behind a reference

let s = String::from("hi");
let r = &s;
r.len();                 // method calls dereference automatically, no (*r).len() needed
```

### No dangling references
```rust
fn bad() -> &String {                 // ❌ E0106: there's nothing to borrow from
    let s = String::from("hi");
    &s                                // s is dropped at the closing `}`
}
fn good() -> String { String::from("hi") }    // ✅ return the owned value instead
```
Returning a reference that points **into an argument** is fine:
```rust
fn first_two(v: &[i32]) -> &[i32] { &v[..2] }   // the result borrows from `v`
```
With a single reference parameter, Rust infers this link automatically. More complex cases need lifetime annotations (Section 07).

## Slices: borrowed views
A slice is `(ptr, len)` pointing into someone else's data. It owns nothing.
```rust
let arr = [1, 2, 3, 4, 5];
let v = vec![10, 20, 30];
let a: &[i32] = &arr[1..4];      // [2, 3, 4]   (also [..2], [2..], [..])
let b: &[i32] = &v;              // a whole Vec as a slice

let s = String::from("hello world");
let hello: &str = &s[0..5];      // &str is a string slice
let lit: &str = "literal";       // a literal is &'static str, pointing into the binary
```

**Idiom: accept the most general borrowed type**

| Instead of | Take | Accepts |
|---|---|---|
| `&String` | `&str` | `&String`, literals, sub-slices |
| `&Vec<T>` | `&[T]` | `&Vec<T>`, `&[T; N]` arrays, sub-slices |
| `&mut Vec<T>` | `&mut [T]` | only when you don't need to `push` / `remove` |

The conversions `&String → &str` and `&Vec<T> → &[T]` happen automatically (deref coercion).

## String essentials
```rust
let mut s = String::new();               // or String::from("…"), "…".to_string()
s.push_str("hello");                     // append a &str
s.push('!');                             // append a char
let t = s.clone() + " world";            // String + &str (moves the left side)
let u = format!("{s} {t}");              // never moves anything
s.len();                                 // length in BYTES (UTF-8)
s.chars().count();                       // length in characters
s.find('l');                             // Option<usize>, a byte index
s.trim(); s.to_uppercase(); s.replace("a", "b");
for c in s.chars() {}
for w in "a  b c".split_whitespace() {}  // each w is a &str into the original
```
- `s[0]` **does not compile**. A UTF-8 character takes 1–4 bytes, so "the n-th char" can't be found in O(1). Use `s.chars().nth(n)`.
- `&s[a..b]` slices by **bytes**. Cutting through the middle of a multi-byte character panics.

## Vec essentials (more in Section 05)
```rust
let mut v: Vec<i32> = Vec::new();        // or vec![1, 2, 3]
v.push(4);
v.pop();                                 // Option<i32>
v.len(); v.is_empty(); v.contains(&3);
v[0];                                    // panics if out of bounds
v.get(10);                               // Option<&i32>, never panics
v.remove(1);                             // removes by index and RETURNS the element (ownership!)
```

## Iterating: by value or by reference
| Loop | Item type | After the loop |
|---|---|---|
| `for x in v` | `T`, moved out | `v` is gone |
| `for x in &v` | `&T` | `v` still usable |
| `for x in &mut v` | `&mut T` (write `*x += 1`) | `v` still usable, modified |

`v.iter()` is the same as `&v`, and `v.iter_mut()` the same as `&mut v`. To get indexes too: `for (i, x) in v.iter().enumerate()`.
Clippy's Section 01 warning "the loop variable `i` is only used to index" meant exactly this: loop over the elements directly.

## Choosing a parameter type
| The function needs to… | Parameter type |
|---|---|
| read the value | `&T`, `&str`, `&[T]`. **This is the default choice** |
| modify the caller's value | `&mut T` |
| keep it (store it, consume it) | `T`, taking ownership |
| hand back newly created data | return `T` (owned) |

## Solidity analogy
| Solidity | Rust (roughly) |
|---|---|
| `Data storage d = data;` (a pointer; writes persist) | `let d = &mut data;` |
| `Data memory d = data;` (a copy) | `let d = data.clone();` |
| `calldata` (read-only view) | `&T`, `&[T]`, `&str` |

## Handy: moving out through a `&mut`
```rust
let old = std::mem::take(&mut s);          // takes s, leaves the default behind ("" / empty Vec / 0)
let old = std::mem::replace(&mut s, new);  // takes s, puts `new` in its place
std::mem::swap(&mut a, &mut b);
```

## Borrow checker errors cheat sheet
| Code | Meaning | Usual fix |
|---|---|---|
| E0382 | use of a moved value | borrow (`&x`) instead of moving, or reorder, or `.clone()` |
| E0499 | two `&mut` borrows at once | use them one after another |
| E0502 / E0506 | `&mut` borrow or assignment while a `&` borrow is alive | reorder so the `&` borrow is last used before; or copy the value out |
| E0505 | move while borrowed | finish using the reference before the move |
| E0507 / E0508 | move out of a reference, index or slice | borrow it (`&v[i]`), `v.remove(i)`, `mem::take`, or `.clone()` |
| E0515 / E0106 | returning a reference to a local | return the owned value |
| E0597 | borrowed value does not live long enough | move the owner to an outer scope, or own the result |
| E0596 / E0384 | mutating something immutable | `let mut`, or a `&mut` parameter |

`rustc --explain E0502` prints a detailed explanation with examples.

## Exercises
**Part A: make it compile** (`tests/fix*.rs`). Each file is broken on purpose.
Read the compiler error and fix it with the smallest change. Don't change what the `assert`s check.
```bash
cargo test --test fix01_use_after_move
```

**Part B: implement** (`src/ex*.rs`). Replace every `todo!()`.
```bash
cargo test --lib          # only src/, works even while tests/fix*.rs is still broken
cargo test --lib ex02
```
Allowed: `for` loops and std methods on `String` / `Vec` / slices (`push`, `remove`, `chars`, `split_whitespace`, `find`, …).
Not yet: iterator chains like `.map`, `.filter`, `.collect`, `.sum`, `.max` (Section 08).

The section is done when `cargo test` passes and `cargo clippy --all-targets` is clean.
