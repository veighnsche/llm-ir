# llmir-sys

Purpose
- Capability traits for host interaction: `Fs, Proc, Env, Time, Json, Hash, Http, Gpu`.
- Provide default `Null*` implementations that return `E_CAP`.
- Bundle as `Sys` to inject into the VM.

Status
- Traits stubbed; feature flags planned for mock backends in tests.

Pipeline role
- Defines host capability facades used by the VM and higher layers.
- Default `Null*` impls intentionally fail with `E_CAP` to surface missing capabilities.
- Actual system bindings or mocks can be injected via the `Sys` bundle.

Capability surfaces (initial)
- `Fs`   — read-only for now; future: write, list, metadata.
- `Proc` — spawn process, later: stdin/stdout piping and status streaming.
- `Env`  — environment variables access.
- `Time` — monotonic time, deadlines.
- `Json` — parse/serialize JSON values.
- `Hash` — stable hashes like SHA-256 for cache keys.
- `Http` — minimal HTTP client surface (GET now, more later).
- `Gpu`  — lightweight hints for GPU affinity/pinning.

Example: injecting a null system into the VM
```rust
use llmir_sys::{Sys, NullFs};

let sys = Sys { fs: Some(&NullFs), ..Default::default() };
// pass `sys` to VM runner once the API accepts it
```

Dev notes
- Keep traits narrow and test-friendly. Prefer strings/bytes for data, avoid heavy types.
- All errors map through `llmir-diag` with `E_CAP` and precise messages.
- Consider per-capability feature flags to include optional deps only when needed.

Testing
- Provide mock implementations gated behind `cfg(test)` or a `mock-sys` feature.
- Conformance tests should exercise both success and `E_CAP` failure paths.

TODOs
- Define per-capability method sets and error codes.
- Provide basic mocks for conformance tests.
