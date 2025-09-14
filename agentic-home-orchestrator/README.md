# Agentic Home Chatbot Orchestrator (LLM-IR Example)

Purpose: a small home chatbot service with an optional Agentic Mode, built to demonstrate the Orchestrator pattern in LLM-IR.

## Highlights
- rails errors with escalation paths (fast/soft/hard/crash)
- ARC (no GC) memory model instincts in examples and patterns
- async tasks and deadlines; safe cancellation via select/deadline
- process workers (llama.cpp), env, time, fs, http, gpu pinning
- bounded channels and SSE re-emit for browser streaming

## How to read this example
1. docs/OVERVIEW.md
2. src/*.pulse (start with `src/orchestrator.pulse`)
3. tests/*

## Canon rules summary (LLM-IR)
- zero indent, one space between atoms, no blank lines
- call-free calls (just heads); prefer pipeline with `pipe`
- module headers include: `(rails)`, `(compact core)`, `(async)` if used, `(caps …)`
- use pipeline: `pipe`, `lam x …`, positional args `$0`/`$1`, ops as symbols
- strings quoted only if they contain spaces; bare numbers; true/false/nil
- first-class comments with `(com kind channel "text")` attach-next
