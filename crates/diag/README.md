# llmir-diag

Purpose

- Diagnostics primitives and reporting helpers.

Public API (stubs)

- Error codes: `E_TYPE, E_ARITY, E_FEATURE_MISSING(x), E_FEATURE_CONFLICT(x), E_CAP, E_COMMENT_*`.
- Span/Path-aware reporting using `miette`/`ariadne`.
- `ResultExt` helpers.

Status

- Compiling stubs; tests to ensure codes and renderers are stable.

Pipeline role

- Central place for error codes and rendering adapters.
- Keeps codes stable across crates; consumers construct `Diag` with a `Code`.

Error code conventions

- `E_TYPE` — type mismatch or unknown type.
- `E_ARITY` — wrong number/shape of arguments for a tag.
- `E_FEATURE_MISSING(x)` — feature/profile not enabled or implemented.
- `E_FEATURE_CONFLICT(x)` — mutually exclusive feature usage.
- `E_CAP` — missing capability; typically returned by `llmir-sys` null impls.
- `E_COMMENT_*` — comment parsing/attachment issues.

Example diagnostic (pseudo)

```log
error[E_ARITY]: expected 2 items in (let …), found 3
  --> input.llmir:1:10
   |
 1 | (let x 1 2)
   |          ^ extra argument
```

Dev notes

- Keep `Diag` small and serializable; integrate with `miette`/`ariadne` only at boundaries.
- All crates should avoid inventing new codes; extend the shared enum instead.

Testing

- Unit tests for formatting and JSON serialization.
- Golden tests for multi-label renders when span wiring lands.

TODOs

- Richer span/label rendering across crates.
- JSON diagnostics flag integration with CLI.
