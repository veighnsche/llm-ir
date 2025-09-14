# llmir-reader

Purpose

- Parse zero-indent, call-free s-expressions into `llmir-ast::Node`.
- Recognizes comment nodes: `(com kind chan "text" …)`.

Public API (stub)

- `parse(src: &str) -> Result<Node, ReaderError>`

Status

- Minimal parser stub; compiles and integrates with CLI.
- Test: parse a minimal module `(mod m (fn f -> i32 () 0))`.

Usage

- Used by `llmirc parse|canon|check|run`.

Pipeline role

- Input: UTF-8 source in zero-indent, call-free s-expr form.
- Output: `llmir-ast::Node` with conservative `Span` info; no semantic validation here.
- Next stages: `llmir-schema` (shape), `llmir-types` (typing), `llmir-canon` (format).

Grammar sketch (informal)

```
sexpr  := '(' head item* ')'
head   := symbol | operator
item   := sexpr | symbol | string | int | bool | nil
comment:= '(com' kind chan string item* ')'
```

Example input

```
(mod m
  (fn f -> i32 ()
    0))
```

Rust usage

```rust
let src = "(mod m (fn f -> i32 () 0))";
let node = llmir_reader::parse(src)?;
println!("{}", llmir_canon::format(&node));
```

Dev notes

- Keep this crate tolerant to whitespace variants but strict about structure.
- Strings and numbers should prefer “shortest spelling” that roundtrips via canon.
- Avoid allocating token streams; a single-pass parser is sufficient for MVP.

TODOs

- Shortest spelling for strings and numbers.
- Positional args `$0`, `$1`, …
- Operators as heads.
- Better error recovery and span fidelity.
