# Rust Tutorial — Plan

Background: C++ / Java / Python / Solidity → Rust from zero.
Style: short theory, lots of runnable code, exercises checked by tests.

## How to work on a section
1. Read `NN_name/README.md` (theory, ~10–15 min).
2. Run the examples: `cd NN_name && cargo run --example <file_name>`.
3. Solve exercises in `NN_name/src/exNN_*.rs`: replace every `todo!()` until `cargo test` is green.
   Don't edit the tests, they are the spec.
   Std methods you'll need (`split_once`, `trim`, `unwrap_or`, …) are in [STD_CHEATSHEET.md](STD_CHEATSHEET.md).
4. Run `cargo clippy` and fix its warnings. This is how you learn idiomatic Rust.
5. Ask Claude to review your solutions. The next section is generated after that,
   adjusted to the mistakes you made.

## Sections
| #  | Section                          | Key topics                                                              | Status |
|----|----------------------------------|-------------------------------------------------------------------------|--------|
| 00 | Intro                            | what Rust is for, strengths/weaknesses, ecosystem, good-to-knows        | ✅ |
| 00 | Setup                            | rustup, cargo, Cursor/VS Code + rust-analyzer                           | ✅ |
| 01 | Basics                           | variables, types, functions, control flow, arrays/tuples, `match`, tests | ✅ |
| 02 | Ownership & Borrowing            | move/copy/clone, `&` / `&mut`, borrow rules, slices, `String` vs `&str` | ✅ |
| 03 | Structs, Enums, Pattern Matching | `struct` + `impl`, enums with data, `match`, `if let`, `Option`         | ✅ |
| 04 | Error Handling                   | `panic!` vs `Result`, `?`, custom errors, `thiserror` / `anyhow`        | 🟡 |
| 05 | Collections & Strings            | `Vec`, `HashMap`, `HashSet`, UTF-8 strings                              | ⬜ |
| 06 | Traits & Generics                | traits, generics, bounds, `derive`, std traits, `impl Trait` vs `dyn`   | ⬜ |
| 07 | Lifetimes                        | `'a` annotations, elision rules, structs holding references             | ⬜ |
| 08 | Closures & Iterators             | `Fn`/`FnMut`/`FnOnce`, iterator adapters, implementing `Iterator`       | ⬜ |
| 09 | Modules, Crates, Testing         | `mod`/`pub`/`use`, workspaces, dependencies, unit/integration/doc tests | ⬜ |
| 10 | Smart Pointers                   | `Box`, `Rc`, `RefCell`, `Weak`, `Drop`, interior mutability             | ⬜ |
| 11 | Concurrency                      | threads, channels, `Arc<Mutex>`, `Send`/`Sync`                          | ⬜ |
| 12 | Async                            | `async`/`await`, futures, `tokio` basics                                | ⬜ |
| 13 | Macros & Unsafe (overview)       | `macro_rules!`, derive macros, `unsafe`, FFI glimpse                    | ⬜ |
| 14 | Final Project                    | CLI app pulling everything together (e.g. a mini token ledger / blockchain) | ⬜ |
| 15 | *(optional)* Rust in Web3        | `alloy` (EVM client), Solana/Anchor, Arbitrum Stylus                    | ⬜ |

## Repo layout
```
Cargo.toml          # workspace: every section is a member crate
00_intro/README.md  # read first
STD_CHEATSHEET.md   # std methods used by the exercises, by type
00_setup/README.md
01_basics/
  README.md         # theory
  examples/*.rs     # runnable demos: cargo run --example <name>
  src/ex*.rs        # implement exercises + tests: cargo test --lib
  tests/fix*.rs     # make-it-compile exercises (from 02): cargo test --test <name>
```

Status: ⬜ not started · 🟡 in progress · ✅ done
