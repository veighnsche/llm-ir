# llmir-canon

Purpose
- Canonicalize/format LLM-IR with zero-indent and single-space separation.
- Attach comments to the next node; comments move with their targets.

Public API (stub)
- `format(node: &Node) -> String`

Status
- Formatting stub for tiny modules; used by CLI `canon`.

Testing
- Prefer golden/snapshot tests for small inputs.

Pipeline role
- Accepts `llmir-ast::Node` and produces a stable textual form.
- No semantic changes; goal is idempotence: `parse(canon(parse(src)))` ≈ `parse(src)`.

Formatting rules (initial)
- Zero indentation; a single space between atoms/items.
- Strings quoted with minimal escaping; integers and booleans in shortest readable form.
- Comments `(com ...)` attach to the immediately following node and travel with it.

Before/after example
```
;; input (informal spacing)
(  mod   m (fn   f -> i32 ()   (ok   0)))

;; canon output
(mod m (fn f -> i32 () (ok 0)))
```

Dev notes
- Keep the printer pure and deterministic; avoid reading external state.
- When adding new atom kinds, define a single place for their printing policy.
- For comments, start with attach-next; attach-prev may be considered later.

TODOs
- Implement “shortest literal” rules (strings, ints, bools) consistently.
- Comment attach and move semantics, including multi-line.
