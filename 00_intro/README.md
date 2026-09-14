# 00 — Intro: What Rust Is and Why

**Rust** is a systems language with C++-level performance and memory safety guaranteed at compile time, with no garbage collector.

- Created by Graydon Hoare (2006), sponsored by Mozilla, **1.0 in May 2015**.
- Governed by the Rust Foundation (AWS, Google, Microsoft, Huawei, Mozilla, …).
- A new stable release every **6 weeks**. **Editions** (2015/2018/2021/2024) bring opt-in breaking changes, and crates on different editions still link together.
- Voted the most "admired/loved" language in the Stack Overflow survey for many years in a row.

## The core idea
| | Memory model | Price you pay |
|---|---|---|
| C++ | manual / RAII | UB: use-after-free, dangling pointers, data races |
| Java / Python | garbage collector | runtime, GC pauses, memory overhead |
| **Rust** | **ownership + borrow checker, checked at compile time** | learning curve, slower compiles |

The compiler tracks who **owns** each value and who **borrows** it. Invalid access is a compile error, not a runtime crash.

## Strengths 💪
- **Speed** on par with C/C++. No GC and no runtime, so latency is predictable and it can be embedded anywhere.
- **Memory safety** in safe Rust: no null, no use-after-free, no buffer overflows, no data races.
  (~70% of serious CVEs at Microsoft and in Chrome are memory-safety bugs. US agencies now recommend memory-safe languages.)
- **Zero-cost abstractions**: iterators, closures and generics compile to the same machine code as hand-written loops.
- **Fearless concurrency**: sharing non-thread-safe data between threads doesn't compile.
- **Expressive types**: enums with data, exhaustive pattern matching, traits, `Option` / `Result` instead of null and exceptions.
- **Tooling**: `cargo` handles build, dependencies, tests, docs and benchmarks in one tool. `clippy`, `rustfmt`, rust-analyzer. Compiler error messages are outstanding, so read them.
- **Targets**: native binaries, WebAssembly, embedded (`no_std`), and easy C FFI.

## Weaknesses ⚠️
- **Steep learning curve.** Expect to "fight the borrow checker" for the first weeks, then it clicks.
- **Slow compilation**, especially release builds and code heavy on generics or macros.
- **Some designs are awkward**: graphs, doubly linked lists, self-referential structs, OOP inheritance (there is none).
- **Async is powerful but complex**: `Pin`, `Send` bounds, and you pick a runtime (in practice: tokio).
- **Small std by design**: JSON, HTTP, async runtime and random numbers all come from third-party crates. Many crates stay at `0.x` for years.
- **Weak spots in the ecosystem**: GUI, ML/data science and game engines are still maturing.
- **Overkill for quick scripts** and throwaway prototypes, where Python wins.
- **No stable ABI**: Rust libraries link statically; for plugins or dynamic linking you go through the C ABI.

## What it's used for (most common first)
| Domain | Real-world examples |
|---|---|
| Backend services / networking | Cloudflare (Pingora proxy), Discord, Dropbox sync engine, AWS services |
| CLI & developer tools | ripgrep, fd, bat, **uv / ruff** (Python tooling), Deno, SWC, Turbopack, Biome, Zed editor |
| Cloud & infrastructure | AWS Firecracker (runs Lambda), Linkerd proxy, Vector |
| OS & low-level | **Linux kernel** (drivers, since 6.1), Windows kernel parts, Android (Bluetooth, Keystore) |
| Browsers | Firefox (Stylo CSS engine), Chrome (font and QR components), Servo |
| Databases & data | TiKV, SurrealDB, Qdrant, InfluxDB 3, Polars, DataFusion |
| **Web3** | Solana (validator + programs), Polkadot SDK, NEAR, **Reth** & **Lighthouse** (Ethereum clients), **Foundry** (forge/cast/anvil), revm, zkVMs (SP1, RISC Zero) |
| WebAssembly | frontend frameworks, plugins, edge functions |
| Embedded / IoT | ESP32, STM32, nRF via embassy |

## Frameworks & crates you'll meet
| Area | Crates |
|---|---|
| Serialization | `serde` (+ `serde_json`, `toml`), the de facto standard everywhere |
| Async runtime | `tokio` (de facto), `smol` |
| Web backend | `axum` (most popular), `actix-web`, `rocket`, `loco` (Rails-like) |
| HTTP / gRPC client | `reqwest` / `tonic` |
| Database | `sqlx` (SQL checked at compile time), `diesel` (ORM), `sea-orm` |
| CLI / TUI | `clap` (arguments), `ratatui` (terminal UI), `indicatif` (progress bars) |
| Errors | `anyhow` (apps), `thiserror` (libraries) |
| Logging | `tracing`, `log` |
| Parallelism | `rayon` (parallel iterators), `crossbeam` |
| Frontend (WASM) | `leptos`, `dioxus`, `yew`, `wasm-bindgen` |
| Desktop | `tauri` (web UI, an Electron alternative), `egui`, `iced`, `slint` |
| Games | `bevy` |
| Embedded | `embassy`, `esp-hal` |
| Data / ML | `polars`, `ndarray`, `candle` (Hugging Face), `burn` |
| Testing | `proptest` (property-based), `insta` (snapshots), `criterion` (benchmarks), `mockall` |
| Interop | `pyo3` + `maturin` (Python extensions), `cxx` / `bindgen` (C++ / C) |
| **Web3** | `alloy` (Ethereum client), `anchor` (Solana), `stylus-sdk` (Arbitrum), `ink!` (Polkadot), `cosmwasm` |

Find crates at [crates.io](https://crates.io) (registry), [docs.rs](https://docs.rs) (docs for every crate), and [blessed.rs](https://blessed.rs) (a curated list of recommended crates).

## Mental map from your languages
| Concept | C++ | Java | Python | Solidity | **Rust** |
|---|---|---|---|---|---|
| Memory | RAII / manual | GC | GC + refcount | EVM | ownership, compile-time checked |
| "No value" | `nullptr` | `null` | `None` | zero value | `Option<T>` |
| Errors | exceptions | exceptions | exceptions | `revert` / `require` | `Result<T, E>` + `?`; `panic!` for bugs |
| Polymorphism | virtual, templates | interfaces, generics | duck typing | inheritance, interfaces | **traits** + generics |
| Inheritance | yes | yes | yes | yes | **none**, use composition + traits |
| Default semantics | copy | reference | reference | depends on location | **move** (a moved-from variable is unusable) |
| Generics | templates (checked at instantiation) | type erasure | — | — | monomorphized, checked at definition via trait bounds |
| Build / packages | CMake + conan / vcpkg | Maven / Gradle | pip / uv | Foundry / Hardhat | `cargo` |

## Good to know
- **"If it compiles, it usually works."** True for memory and threading bugs, not for logic bugs.
- **Immutable by default**, and mutability is visible in signatures (`&mut`).
- **RAII** like C++: destructors (`Drop`) run deterministically when the owner goes out of scope.
- **`unsafe`** exists for raw pointers and FFI, as small audited islands. The standard library is built on it, and your code rarely needs it.
- **Macros** (`println!`, `vec!`, `#[derive(Debug)]`) operate on syntax trees and are hygienic. They are not the C preprocessor.
- **Naming is enforced by warnings**: `snake_case` for functions and variables, `CamelCase` for types, `SCREAMING_CASE` for constants.
- **Channels**: stable (use this) / beta / nightly (experimental features).
- Community slang: *Rustaceans*. The mascot is Ferris the crab 🦀.

## When to choose Rust, and when not
✅ performance or reliability is critical, long-lived infrastructure, CLI tools, blockchain nodes and zk, WASM, replacing C/C++ components, concurrency-heavy services.
❌ quick scripts, exploratory data science, CRUD MVPs on a tight deadline with a team new to Rust, GUI-heavy desktop apps.

## Extra resources (optional, when stuck)
- [The Rust Book](https://doc.rust-lang.org/book/): the official book (`rustup doc --book`)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/): Google's course for experienced developers
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) and [Rustlings](https://github.com/rust-lang/rustlings): small exercises
- [std docs](https://doc.rust-lang.org/std/): search is excellent
