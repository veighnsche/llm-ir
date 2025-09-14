# llmir-lower

Purpose
- Lower AST to a tiny bytecode for the reference VM.

Public API (stub)
- `lower(node: &Node) -> Result<Vec<Instr>, Diag>`
- `Instr` enum is intentionally small (ints, add, return, rails placeholders).

Status
- Compiling skeleton; rails propagation TODO to match VM mapping.

Pipeline role
- Translates a validated AST into a compact, VM-friendly bytecode.
- Keeps spans where possible for error mapping (future: debug map).
- Avoids type decisions; relies on `llmir-types` to pre-validate.

Instruction set sketch (initial)
- `Nop` — placeholder
- `ConstI i64` — push integer
- `AddI` — pop two ints, push sum
- `Ret` — return top of stack
- `Ok`/`Fast`/`Soft`/`Hard`/`Crash` — wrap top-of-stack into rails result

Example (informal)
```
; (ok 0)
ConstI 0
Ok
Ret
```

Dev notes
- Define all instructions in one enum; prefer explicit stack effects in comments.
- Keep lowering deterministic and free of side effects.
- Leave room for later backends (WASM/LLVM) by modeling a clean IR.

Testing
- Unit tests per construct: constants, add, simple ok/crash.
- Golden tests for short lowering sequences are acceptable.

TODOs
- Add lowering rules for arithmetic, control, and rails constructors.
- Annotate instructions with spans for diagnostics mapping.
