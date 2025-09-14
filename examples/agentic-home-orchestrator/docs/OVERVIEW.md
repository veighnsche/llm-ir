# Overview

This example demonstrates an Agentic Home Chatbot Orchestrator built with LLM-IR.
It shows how to:

- spawn `llama.cpp` workers as external processes
- parse NDJSON token streams from stdout
- fanout to SSE for browser subscribers
- cancel safely via deadlines
- maintain session state with a tiny LRU sketch

## What it does

- Orchestrator reads config, spawns N workers on demand, and exposes a streaming route
- Worker module consumes process stdout/err and logs tokens deterministically
- SSE bridge re-emits tokens to clients
- An optional agent loop runs a bounded 2-step plan/tool/observe cycle

## Profiles and caps per module

- `src/orchestrator.pulse`: (rails) (compact core) (async) (caps fs proc env time json hash http gpu)
- `src/worker_spawn.pulse`: (rails) (compact core) (async) (caps proc time log)
- `src/sse_bridge.pulse`: (rails) (compact core) (async) (caps http)
- `src/agent_loop.pulse`: (rails) (compact core) (async) (caps time log json)

## Streaming

- Input: NDJSON lines from `llama.cpp` (newline-delimited JSON per token or chunk)
- Output: SSE events to browsers

Framing note: NDJSON is line-framed; SSE is `data:` line framed with blank line separators.
The bridge preserves order and backpressure; cancellation is handled via deadlines in `select` blocks.
