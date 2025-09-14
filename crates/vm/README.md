# llmir-vm

Purpose
- Minimal reference interpreter for the lowered bytecode.

Public API (stub)
- `run(code: &[Instr]) -> i32`

Behavior (placeholder)
- Integers, addition, and return codes.
- Rails mapping: `ok -> 0`, others map to `1/2/10/255` (subject to change).

Status
- Compiling stub; smoke tests for `(ok 0)` path planned.

Pipeline role
- Executes `llmir-lower` bytecode with a tiny stack machine.
- For now: integers, add, rails return codes mapping to process exit.

Execution model (initial)
- Stack-based, single frame.
- Instructions are synchronous; async profile integrates later via `llmir-asyncx`.
- Capabilities (fs/proc/env/...) accessed through injected `llmir-sys::Sys`.

Example
```
; (ok 0)
ConstI 0
Ok
Ret
; => exit 0
```

Dev notes
- Keep interpreter small and predictable; prioritize readability over speed.
- Map rails severities to exit codes in one place to keep policy centralized.
- Separate pure execution from I/O (capabilities) for testability.

Testing
- Smoke tests for sev-to-exit mapping and basic arithmetic.
- Later: capability mocks to simulate fs/http/etc.

TODOs
- Extend with arrays, strings, channels (later phases).
- Integrate capability `Sys` surface for caps-based ops.
