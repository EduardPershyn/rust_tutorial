# 04 — Error Handling

Rust has no exceptions. A failure is either a **bug** (`panic!`, the program stops) or an **expected outcome** (`Result`, an ordinary return value the caller must deal with).

```bash
cd 04_error_handling
cargo run --example results         # also: custom_errors, panic
cargo test --test fix01_question_mark_return
cargo test --lib
cargo test                          # everything
```

## Two kinds of failure
| | `panic!` | `Result<T, E>` |
|---|---|---|
| Meaning | a **bug**: "this should be impossible" | an **expected** failure: bad input, missing file, low balance |
| What happens | the thread stops and prints a message; `main` exits with code 101 | the function returns `Err(…)`, and the caller decides |
| Examples | index out of bounds, `unwrap()` on `None`, overflow in a debug build | `"abc".parse::<i32>()`, a withdrawal larger than the balance |
| Solidity | `assert` / `Panic(0x11)` | `require` / `revert CustomError(…)` |

```rust
panic!("config corrupted: {reason}");      // stop: a bug
unreachable!("state was checked above");   // "this line can't run"
v[10];                                      // implicit panic if out of bounds
opt.unwrap();                               // implicit panic on None / Err
opt.expect("user was validated earlier");  // the same, with a message saying WHY it can't fail
```
Run `cargo run --example panic` to see what a panic looks like. Prefix the command with `RUST_BACKTRACE=1` to see where it came from.

## `Result<T, E>`
```rust
enum Result<T, E> {      // from std, always in scope, just like Option
    Ok(T),               // success, carrying the value
    Err(E),              // failure, carrying the error
}

let r: Result<i32, std::num::ParseIntError> = "42".parse::<i32>();
match r {
    Ok(n) => println!("got {n}"),
    Err(e) => println!("bad input: {e}"),
}
```

| | How errors travel | Visible in the signature? |
|---|---|---|
| C++ / Python | exceptions, invisible control flow | no |
| Java | checked exceptions | yes (`throws`), but often wrapped away |
| Solidity | `revert` undoes the whole transaction; `try/catch` only for external calls | no |
| **Rust** | a **return value** | **always**: `-> Result<u32, BankError>` |

## Handling a `Result`
| You want | Write |
|---|---|
| handle both cases | `match r { Ok(v) => …, Err(e) => … }` |
| only the success case | `if let Ok(v) = r { … }` |
| the value or a fallback | `r.unwrap_or(0)`, `r.unwrap_or_default()` |
| just check | `r.is_ok()`, `r.is_err()` |
| drop the error: Result → Option | `r.ok()` |
| turn None into an error: Option → Result | `opt.ok_or(MyError::Missing)` |
| replace the error type | `r.map_err(MyError::Parse)` (a tuple variant used as a function, see below) |
| pass it up to the caller | `r?` |
| crash (tests, true invariants) | `r.unwrap()`, `r.expect("why")`, `r.unwrap_err()` |

## The `?` operator
```rust
fn read_port(s: &str) -> Result<u16, ParseIntError> {
    let port = s.trim().parse::<u16>()?;   // Err → return it right now; Ok(v) → port = v
    Ok(port)
}
```
`value?` is shorthand for:
```rust
match value {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),   // ← converts the error type if a `From` impl exists
}
```
Rules:
- `?` only works **inside a function that returns `Result` (or `Option`)**. In any other function it's E0277.
- `?` on an `Option` needs a function returning `Option`, and `?` on a `Result` needs one returning `Result`. To mix them, convert first: `opt.ok_or(err)?` or `result.ok()?`.
- The success value at the end still needs wrapping: `Ok(port)`. For "no value", write `Ok(())`. It's the tail expression rule from Section 01 again.

## Don't ignore a `Result`
`Result` is `#[must_use]`: calling `withdraw(…);` and discarding the result is a compiler warning. It's the Rust version of Solidity's **unchecked `call` return value** bug.
```rust
withdraw(&mut acc, 50);                    // ⚠️ warning: unused `Result` that must be used
withdraw(&mut acc, 50)?;                   // ✅ propagate
let _ = withdraw(&mut acc, 50);            // explicit "I don't care": avoid for anything important
```

## Your own error type
An enum, where each variant is one failure mode with its data. It's like Solidity's `error InsufficientFunds(uint needed, uint available)`:
```rust
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BankError {
    InsufficientFunds { needed: u64, available: u64 },
    AccountNotFound(String),
    ZeroAmount,
}
```

Two recipes to copy. They use trait syntax, which is explained in Section 06; for now treat them as templates.

**1. A human-readable message (`Display`)** gives you `{}` printing and `.to_string()`:
```rust
impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { needed, available } => {
                write!(f, "insufficient funds: need {needed}, have {available}")
            }
            BankError::AccountNotFound(name) => write!(f, "account not found: {name}"),
            BankError::ZeroAmount => write!(f, "amount must be positive"),
        }
    }
}

impl std::error::Error for BankError {}    // marks it as an error type (needs Debug + Display)
```
`write!(f, …)` works like `format!`, but writes into the formatter.

**2. Automatic conversion for `?` (`From`)**: when a function returns `Result<_, ConfigError>` and calls something failing with `ParseIntError`, `?` needs to know how to convert:
```rust
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    BadNumber(std::num::ParseIntError),
    MissingKey(String),
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(e: std::num::ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn port(s: &str) -> Result<u32, ConfigError> {
    Ok(s.parse::<u32>()?)       // ParseIntError → ConfigError::BadNumber automatically
}
```
Without the `From` impl, `?` fails to compile. Depending on how the call is written, the message is E0277 "`?` couldn't convert the error to `ConfigError`" or E0271 "expected `ConfigError`, found `ParseIntError`". Both mean the same thing. The one-off alternative is `.map_err(ConfigError::BadNumber)?`, which works because a tuple variant like `ConfigError::BadNumber` is itself a function from `ParseIntError` to `ConfigError`.

## `Box<dyn Error>`: "any error" for applications
```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {    // main may return a Result too
    let n: i32 = "42".parse()?;              // ParseIntError  → boxed automatically
    let cfg = load_config("app.cfg")?;       // ConfigError    → boxed too (it implements Error)
    println!("{n} {cfg:?}");
    Ok(())
}                                            // on Err: prints `Error: …` and exits with code 1
```
It's convenient, but callers can no longer `match` on specific variants. Use your own enum in libraries and `Box<dyn Error>` in applications, prototypes and `main`.

## No `revert`: check first, then change
In Solidity, a `revert` rolls back **every** state change in the transaction. In Rust, `?` returns immediately, and **changes already made stay made**:
```rust
fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), BankError> {
    self.withdraw(from, amount)?;    // money is gone from `from`...
    self.deposit(to, amount)?;       // ...and if `to` doesn't exist, it has vanished 💥
    Ok(())
}
```
Validate **everything** first (both accounts exist, the balance is enough) and mutate only when nothing can fail any more.

## Real projects: `thiserror` and `anyhow`
Two crates remove the boilerplate above. You'll add dependencies in Section 09; to try them now in a scratch project: `cargo new try_errors && cd try_errors && cargo add thiserror anyhow`.
```rust
#[derive(Debug, thiserror::Error)]           // thiserror: libraries. It generates Display + Error + From
pub enum ConfigError {
    #[error("bad number: {0}")]
    BadNumber(#[from] std::num::ParseIntError),
    #[error("missing key: {0}")]
    MissingKey(String),
}

fn main() -> anyhow::Result<()> {            // anyhow: applications. Like Box<dyn Error>, plus context
    let port: u16 = "80".parse()?;
    anyhow::ensure!(port != 0, "port must not be zero");   // like require(...)
    Ok(())
}
```

## Testing errors
```rust
assert_eq!(parse_age("abc"), Err("not a number: abc".to_string()));   // needs PartialEq on the error
assert!(matches!(parse_config(""), Err(ConfigError::MissingKey(_))));  // when you only care about the variant
assert_eq!(err.to_string(), "account not found: bob");                 // test the Display message

#[test]
#[should_panic(expected = "division by zero")]   // passes only if the test panics with this text
fn divide_by_zero_panics() { divide(1, 0); }

#[test]
fn uses_question_mark() -> Result<(), ParseIntError> {   // a test can return Result and use ?
    let n: i32 = "5".parse()?;
    assert_eq!(n, 5);
    Ok(())
}
```

## Exercises
**Part A: make it compile / pass** (`tests/fix*.rs`), with the same rules as before. `fix04` and `fix05` compile already: make their tests pass.

**Part B: implement** (`src/ex*.rs`):
- `ex01_results`: parsing with `Result`, `?`, `ok_or`, collecting errors vs stopping at the first one
- `ex02_bank`: a custom error enum with `Display`, and a bank whose `transfer` must be atomic
- `ex03_config`: a config parser with a `From` conversion, so `?` does all the work

New std methods used here are listed in [STD_CHEATSHEET.md](../STD_CHEATSHEET.md) under `Result` and `&str`.
Not yet: closures, e.g. `map_err(|e| …)`. Pass a variant instead: `map_err(ConfigError::BadNumber)`.

The section is done when `cargo test` passes and `cargo clippy --all-targets` is clean.
