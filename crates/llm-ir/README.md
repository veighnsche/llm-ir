# llm-ir (tests)

Purpose

- Workspace-level integration tests for the pipeline and system surfaces.

Contents

- `tests/pipeline.rs`: end-to-end stubs across crates.
- `tests/sys_null.rs`: exercising `Null*` sys capabilities.

Quickstart

- `cargo test -p llm-ir`

Scope

- This crate exists solely for integration tests that span multiple crates.
- Prefer unit tests in individual crates; use this only when cross-crate wiring matters.

How to add a new pipeline test

1. Create a file in `tests/`, e.g. `tests/my_case.rs`.
2. Arrange: prepare a small LLM-IR source string and parse it via `llmir-reader`.
3. Act: run through `llmir-schema` → `llmir-types` → `llmir-lower` → `llmir-vm` as needed.
4. Assert: check success/failure, diags, or VM exit code.

Example skeleton

```rust
use llmir_reader as reader;
use llmir_canon as canon;

#[test]
fn pipeline_min() {
    let src = "(mod m (fn f -> i32 () 0))";
    let node = reader::parse(src).expect("parse");
    let _ = canon::format(&node);
    // add schema/types/lower/vm checks as they mature
}
```

Dev notes

- Keep tests tiny and focused; prefer single-page sources.
- Avoid relying on the host environment (network, files); use `llmir-sys` mocks when available.
- Name tests by feature profile when appropriate (e.g., `rails_ok_path`).

Notes

- Not a library or binary; acts as a test crate only.
