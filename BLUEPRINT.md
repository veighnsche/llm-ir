# LLM-IR Rust Monorepo Blueprint

Goals:
- Workspace monorepo with clear layering.
- Compiling stubs + unit-test scaffolds.
- Docs that map to the LLM-IR cheat sheet (rails, compact, async, caps, ARC, comments).
- Room for a tiny reference VM first; WASM/LLVM backends later.

Crates:
- ast: core data types (Node, Atom, Span, Path)
- reader: zero-indent s-expr reader, comment nodes `(com ...)`
- schema: tag registry, arity/shape validator
- canon: canonicalizer (shortest literals, comment attach)
- types: type rules (prims, struct/sum, res<T>, rails typing)
- lower: lower AST -> bytecode IR
- vm: reference VM (rails, arrays, strings, channels later)
- asyncx: tasks/chan/timer/select model (stubs)
- sys: capability facades: fs/proc/env/time/json/hash/http/gpu (traits + mocks)
- diag: diagnostics (codes, spans, renderers)
- cli: binary: `llmirc` (parse | canon | check | run)

Notes:
- Prefer ARC patterns, no GC.
- Keep initial implementations minimal; leave TODOs and tests for growth.
