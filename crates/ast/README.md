# llmir-ast

Purpose

- Core data types for the LLM-IR abstract syntax tree.

Public API (stubs)

- Types: `Span`, `Tag`, `Atom`, `NodeOrAtom`, `Node`, `Path`, `Severity`, `ResTy<T>`.
- All types serialize/deserialize with `serde` where appropriate.

Status

- Minimal, compiling stubs.
- Unit test: basic construction and serde JSON roundtrip.

Usage

- Add dependency in workspace crates: `llmir-ast` (path = "../ast").
- Construct nodes, traverse `Node.items`, or format via `llmir-canon`.

Design notes

- `Path` is a simple vector of segments for addressing (e.g., `mod/fn[name]/…`). It is not
  tied to the parser; higher layers may construct paths based on symbol tables.
- `ResTy<T>` marks `res<T>` typing for the Rails profile; it’s a zero-sized marker type that
  helps document intent in type signatures and tests.

Data model quick reference

- `Atom`: symbols, strings, integers, booleans, and `nil`.
- `NodeOrAtom`: a heterogeneous list item (either nested `Node` or an `Atom`).
- `Node`: an s-expression with a `head: Atom`, `items: Vec<NodeOrAtom>`, and `span: Span`.
- `Tag`: convenience enum for well-known heads used by validators; you may still see
  free-form symbol heads in early phases.
- `Span { start, end }`: byte offsets into the source; file/line/col are deferred to diagnostics.

Example

```rust
use llmir_ast::{Atom, Node, NodeOrAtom, Span};

let n = Node {
    head: Atom::Sym("ok".into()),
    items: vec![NodeOrAtom::Atom(Atom::Int(0))],
    span: Span { start: 0, end: 0 },
};
```

Dev loop

- Keep this crate free of heavy dependencies; it should remain a lean data model.
- Add helpers in separate crates (e.g., builders) if ergonomics are needed.
- Changes here ripple to all crates; prefer additive evolution.

Testing

- Roundtrips via `serde_json` ensure schema stability for simple cases.
- Add structural tests when introducing new atoms or fields.

TODOs

- Consider richer `Span` (file id, line/col) once diagnostics are wired.
- Add helpers for building nodes safely.
