# llmir-types

Purpose
- Type environment skeleton for primitives and `res<T>`.
- Rails typing placeholders: ctors and combinator signatures.

Public API (stub)
- `typecheck(node: &Node) -> Result<(), Diag>`

Rails model (planned)
- Constructors: `ok/fast/soft/hard/crash : T -> res<T>`
- Combinators: `bind`, `map`, `orfast`, `orsoft`, `fold`, `join` (signatures only for now).

Status
- Compiling stubs only; logic behind feature flags will use `unimplemented!()`.

Pipeline role
- Consumes a shape-checked AST and assigns types or emits diagnostics.
- For MVP, validates presence of primitives and placeholders for `res<T>`.
- Rails profiles influence operator signatures and error propagation rules.

Type system sketch (initial)
- Primitive kinds: `i32`, `i64`, `bool`, `str`, `unit`, collections later.
- Result kind: `res<T>` with severities `ok|fast|soft|hard|crash`.
- Functions: `(fn f -> T (args...) body)` yields type `args -> T`.
- Lambdas: `(lam (args...) body)` similar to `fn` but anonymous.

Example signatures (informal)
```
ok    : T -> res<T>
fast  : T -> res<T>
soft  : T -> res<T>
hard  : T -> res<T>
crash : T -> res<T>

bind  : res<A> (A -> res<B>) -> res<B>
map   : res<A> (A -> B)      -> res<B>
join  : res<res<A>>          -> res<A>
```

Dev notes
- Keep algorithms simple and explicit; prefer readable errors over clever inference.
- Statically shape-driven: rely on `llmir-schema` invariants to simplify checks.
- Gate unimplemented features with cargo feature flags and `unimplemented!()`.

Testing
- Positive/negative typing cases for constructors and simple combinators.
- Ensure diagnostics carry codes and point at the right spans.

TODOs
- Define primitive types and collections.
- Implement rails typing rules and error propagation later.
