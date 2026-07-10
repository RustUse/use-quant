# RustUse Crate Template

Use this checklist when adding a new focused crate or expanding the `use-quant` facade.

## Target Layout

```text
crates/use-example/
  Cargo.toml
  README.md
  examples/
    basic_usage.rs
  src/
    lib.rs
    prelude.rs
    error.rs
  tests/
    basic_usage.rs
```

`error.rs` and `examples/` are optional, but new crates should default toward a small example and at least one integration test.

## Cargo.toml Pattern

```toml
[package]
name = "use-example"
version = "0.1.0"
publish = true
keywords = ["", "", "", "", ""]
description = "Utility-first example helpers for `RustUse`"
homepage = "https://rustuse.org/use-quant/use-example"
documentation = "https://docs.rs/use-example"
readme = "README.md"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
categories.workspace = true

[package.metadata.docs.rs]
all-features = true

[dev-dependencies]
proptest.workspace = true

[lints]
workspace = true
```

Checklist:

- Keep package metadata inherited from the workspace wherever possible.
- Default new crates to `publish = false` until they are intentionally part of a release wave.
- Prefer test-only dependencies in `[dev-dependencies]`.
- Use a utility-first description that matches the existing crate wording.

## src/lib.rs Pattern

```rust
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod prelude;

pub use crate::{ExampleError, example_helper};
```

Checklist:

- Re-export the focused public API at the crate root.
- Add a `prelude` only for items that improve ergonomics without hiding the API shape.
- Use `try_new`-style validated constructors when accepting external or user-provided numeric input.

## README Structure

Keep crate README files short and consistent.

Required sections:

- title and one-line utility-first summary
- `Install`
- `Foundation`
- `When to use directly`
- `Scope`
- `Status`

Guidelines:

- Keep examples runnable as doctests.
- Backtick product names like `RustUse` when the README is included in rustdoc.
- Make `Scope` explicit about what is intentionally out of scope.
- Match the focused crate wording to the facade crate instead of inventing a second naming scheme.

## Facade Checklist

If a new focused crate should be available through `use-quant`, also update:

- `crates/use-quant/Cargo.toml` features and optional dependencies
- `crates/use-quant/src/lib.rs` root re-exports and nested module re-exports
- `crates/use-quant/src/prelude.rs`
- `crates/use-quant/tests/`
- `crates/use-quant/examples/`
- root `README.md`
- Mark feature-specific facade examples and integration tests with `required-features`.

## Testing Checklist

- Add unit tests for each public function or method that introduces logic.
- Add an integration test for the intended crate-level workflow.
- Add property tests when the crate exposes invariants, overflow boundaries, or algebraic identities.
- Prefer exact assertions when values are integer-safe; otherwise use a small tolerance helper.

## Validation Checklist

Run the full workspace suite before opening a pull request:

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```
