# 00 — Setup

## 1. Toolchain
You already have Rust **1.83** (Nov 2024). This tutorial uses **edition 2024**, which needs **1.85 or newer**. Update:

```bash
rustup update stable
rustup component add clippy rustfmt rust-src
rustc --version        # should now be well above 1.85
cargo --version
```

| Tool      | What it is                                   | Analog |
|-----------|----------------------------------------------|--------|
| `rustup`  | toolchain manager (versions, components)     | `nvm`, `pyenv` |
| `rustc`   | the compiler (rarely called directly)        | `g++`, `javac` |
| `cargo`   | build tool + package manager + test runner   | `npm` + `maven`, `forge` |
| `clippy`  | linter with idiom suggestions                | `clang-tidy`, `solhint` |
| `rustfmt` | formatter (`cargo fmt`)                       | `black`, `prettier` |

## 2. Editor (Cursor / VS Code)
The CLI is `cursor` for Cursor and `code` for VS Code:
```bash
cursor --install-extension rust-lang.rust-analyzer      # required: completion, types, errors
cursor --install-extension tamasfe.even-better-toml     # optional: Cargo.toml support
cursor --install-extension vadimcn.vscode-lldb          # optional: debugger
```
Or, inside the editor: `Ctrl+Shift+X` → search `rust-analyzer` (publisher: rust-lang) → Install.

Open the **repo root** (`rust_tutorial/`) in the editor. The root `Cargo.toml` is a workspace,
so rust-analyzer sees every section.

Optional: to run clippy instead of plain check on save, add this to `.vscode/settings.json`:
```json
{ "rust-analyzer.check.command": "clippy" }
```

Tip: turn on inlay hints (on by default). They show the types Rust infers, which helps a lot at the start.

## 3. Verify
```bash
cd 01_basics
cargo run --example variables_types
cargo test        # tests FAIL with "not yet implemented", which is expected: that's your homework
```

## 4. Optional
```bash
git init && git add . && git commit -m "start"   # track your progress
rustup doc --book     # "The Rust Book" offline
rustup doc --std      # std library docs offline
```
Online playground: https://play.rust-lang.org
