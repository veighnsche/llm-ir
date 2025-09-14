# Design

## Architecture
- Orchestrator: owns process + async orchestration and config. It spawns `llama.cpp` workers and wires streams to consumers.
- Worker spawner: small lifecycle helper that reads stdout/stderr and applies a deadline kill as a safety valve.
- SSE bridge: minimal façade to open, send, and close SSE streams for front-end clients.
- Agent loop: optional bounded agent mode (2 deterministic steps) for plan/tool/observe tracing.

## Backpressure and cancellation
- Backpressure is implicit via stream processing and SSE send sequencing.
- Cancellation is handled via `select` with `on_deadline (deadline.ms ...)` to kill workers deterministically.

## Security via caps
- Each module declares only the capabilities it needs using `(caps ...)`.
- Tests include a negative caps check to demonstrate E_CAP on missing permissions.
- Avoids ambient authority; processes, http, fs, env, time, gpu are all opt-in.
