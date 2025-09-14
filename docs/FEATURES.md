# Features

- AST: Node/Atom/Span, Path
- Reader: zero-indent s-expr, comment nodes `(com ...)`
- Schema: tag/arity/shape checks
- Canon: canonicalizer (shortest literals, comment attach)
- Types: primitives, `res<T>`, rails typing placeholders
- Lower: bytecode IR
- VM: reference interpreter stub
- Asyncx: task/chan/timer/select stubs
- Sys: capability traits + null impls (E_CAP)
- Diag: diagnostics (codes, spans)
- CLI: `llmirc` parse | canon | check | run
