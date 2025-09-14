# llmir-schema

Purpose

- Table-driven arity/shape checks for core tags: `mod, fn, lam, let, set, match, arm, struct, sum, com`.

Public API (stub)

- `check_shapes(node: &Node) -> Result<(), Diag>`

Status

- Stubs for profile-gated shapes; unsupported features return `E_FEATURE_MISSING`.

Usage

- Called during `llmirc check` after parse and before typecheck.

Pipeline role

- Ensures syntactic forms have the right arity and structure before deeper checks.
- Annotates diagnostics with codes and spans via `llmir-diag`.
- Feature profiles gate certain tags and shapes (rails, compact, async, caps).

Shape policy (initial)

- Be permissive with unknown tags (treat as `Other`) but strict with core tags.
- Core tags (non-exhaustive): `mod`, `fn`, `lam`, `let`, `set`, `match`, `arm`, `struct`, `sum`, `com`.
- Comments `com` are allowed in lists but do not affect arity of enclosing nodes.

Example shapes (informal)

```
mod    := (mod sym item*)
fn     := (fn sym -> type (param*) body)
lam    := (lam (param*) body)
let    := (let sym expr body)
set    := (set sym expr)
match  := (match scrutinee arm+)
arm    := (arm pat body)
struct := (struct sym (field:type)*)
sum    := (sum sym (ctor (field:type)*)*)
com    := (com kind chan "text" item*)
```

Dev notes

- Keep this crate table-driven; avoid hardcoding shapes in code when possible.
- Wire feature checks early and return `E_FEATURE_MISSING` for unsupported profiles.
- When in doubt, prefer an error that points to the precise offending child span.

Testing

- Positive/negative cases per tag; group by profile.
- Golden diagnostics (message, code, and span labels) where useful.

TODOs

- Fill out shape tables and diagnostics (spans, paths).
- Wire feature gates: rails, compact, async, caps.
