# Run Guide

## Assumptions
- `llama.cpp` binary available on `PATH`
- A local GGUF model path (e.g., `/models/llama-3.1.gguf`)

## Environment
- `MODEL_PATH` – absolute path to the GGUF model
- `GPU_MASK` – optional GPU selection (e.g., `0` or `0,1`)
- `HOST` and `PORT` – where to serve SSE (e.g., `127.0.0.1`, `8080`)

## Steps
1) Configure `src/config_example.json`
2) Start the orchestrator runtime for `src/orchestrator.pulse`
3) Hit the SSE endpoint from a terminal

Example curl (SSE long-poll):
```bash
curl -sN "http://${HOST:-127.0.0.1}:${PORT:-8080}/sse?sid=sid-1"
```

## Troubleshooting
- Missing caps (E_CAP): ensure each module declares required `(caps ...)`
- Rails escalations:
  - fast: short-circuit with `ok`/`err`
  - soft: recoverable issues that continue
  - hard: immediate abort (`hard "reason"`)
  - crash: unhandled or fatal condition
- If `llama.cpp` not on PATH, set an absolute path or fix `proc.spawn` args
- If SSE appears stalled, check deadlines and any backpressure logs
