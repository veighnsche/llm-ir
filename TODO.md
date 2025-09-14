# LLM-IR Development TODO (Living)

Milestones
- v0.1 (MVP pipeline green)
- v0.2 (Reader/schema/types growth; rails semantics; VM mappings)

v0.1 Tasks (ordered)
1. Meta crate `llm-ir` with pipeline smoke test
   - Accept: `cargo test -p llm-ir` passes a test that runs reader → schema → types → lower → vm on a minimal module.
2. Diagnostics unit tests
   - Accept: tests assert `feature_missing("async")` and `cap()` codes/messages.
3. Sys null backends
   - Implement `Null` provider implementing all capability traits with default E_CAP returns.
   - Accept: unit test calls methods and receives `ECap` codes.

v0.2 Tasks (sketch)
4. Reader expansion: strings/ints/$0/ops-as-head parsing; `(com ...)` comment node recognition
   - Accept: parse tests cover minimal variants; comments attach-next preserved in AST.
5. Schema tables: arity/shape checks for `mod, fn, lam, let, set, match, arm, struct, sum, com`
   - Accept: positive/negative shape tests with `EArity`/`EType`.
6. Types: `res<T>` rails typing and combinator signatures
   - Accept: typecheck tests for `ok/fast/soft/hard/crash` ctors to `res<_>`.
7. Lower/VM: rails opcode propagation, exit-code mapping
   - Accept: `(ok 0)` → rc 0; `(fast x)` → rc 1; `(soft x)` → rc 2; `(hard x)` → rc 10; `(crash x)` → rc 255.

Refactor/Infra
- Keep CLI wiring stable; add `--json-diag` later.
- Maintain docs in `docs/` in sync with features.
