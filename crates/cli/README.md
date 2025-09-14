# llmirc (CLI)

Purpose
- Binary CLI for the LLM-IR toolchain.

Commands (stubs)
- `parse <file>`: parse to AST and print a short summary.
- `canon <file>`: canonicalize formatting and print.
- `check <file>`: run schema + types; prints `ok` on success.
- `run <file>`: lower + execute in VM and print exit code.

Flags
- `--help` (via clap). Planned: `--json-diag`, `--no-color`.

Status
- Wires reader → canon → schema → types → lower → vm.

Pipeline wiring
- `parse`: reader → prints byte count of formatted AST.
- `canon`: reader → canon → stdout.
- `check`: reader → schema → types; prints `ok` when both succeed.
- `run`: reader → lower → vm; prints exit code.

Examples
```
cargo run -p llmirc -- parse examples/agentic-home-orchestrator/examples/min.llmir
cargo run -p llmirc -- canon examples/.../min.llmir
cargo run -p llmirc -- check examples/.../min.llmir
cargo run -p llmirc -- run examples/.../min.llmir
```

Flags roadmap
- `--json-diag` — print structured diagnostics for editor integration.
- `--no-color` — disable ANSI coloring in terminals without support.
- `--profile rails|compact|async` — select feature profiles (later).

Dev notes
- Keep CLI thin; most logic should live in library crates.
- Treat input as UTF-8; pass spans through as-is.
- Exit with non-zero code when a subcommand fails.

Testing
- Prefer golden/snapshot tests for small inputs.

Quickstart
- `cargo run -p llmirc -- --help`
- `cargo run -p llmirc -- parse examples/...`
