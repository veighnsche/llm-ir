# LLM-IR — A Token-Cheap Programming Language for LLMs

LLM-IR is a programming language that **isn’t designed for humans at all**.
It’s a **canonical, textual AST**: compact, deterministic, and trivial for an LLM to generate or read.

The goal:

* **LLM-friendly** → minimal token footprint, no ambiguous syntax.
* **Compiler-friendly** → direct parse into a typed IR, single-pass semantics.
* **Deterministic** → unique canonical form; no stylistic variance.
* **Executable** → compiles to bytecode, LLVM, or WASM; used for real computation.

---

## 📡 Streaming to Clients

Workers emit raw text or NDJSON on stdout. An orchestrator reads NDJSON lines and re-emits Server-Sent Events (SSE) to browsers.

* Minimal SSE frame (one token):

  ```
  event: token
  data:<json>

  ```

* Cancellation: client closes the SSE stream; server cancels the task and ensures the child process is terminated within a deadline.

Framing modes:

* Inputs: `raw | ndjson` from `proc.stdout`.
* Outputs: `sse` (browser), `ws` (future profile).

---

## 📌 Status

This repository currently contains the evolving specification and conformance surface for LLM-IR. A reference implementation (parser, canonicalizer, VM, backends) is planned but not yet present in this repo.

If you want to explore the language today, start with the docs:

- `docs/features.md` — high-level feature overview and scope
- `docs/spec-surface.md` — grammar and semantics surface, node inventory, contracts
- `docs/conformance-pack.md` — end-to-end conformance scenarios (positive/negative)

Contributions are welcome on the spec, examples, and conformance pack.

---

## 🎯 Target Use-Case

LLM-IR is motivated by a Home Chatbot Service with an optional Agentic Mode:

* Small LLM emits a full service in one shot: tasks, process supervision, GPU pinning, bounded queues, cancellation, structured logs.
* Deterministic, token-cheap IR ensures repeatable builds and easy diffing.
* Profiles add capabilities as needed: async scheduling, OS/process control, GPU affinity, filesystem, hashing, timers, networking.

This shapes LLM-IR to be predictable for tools and learnable for small models.

---

## ✨ Features

* **Canonical S-expression format**: one space, one newline rule, no trivia comments. Use first-class `(c ...)` nodes for docs.
* **Minimal node set**: `mod, fn, type, struct, sum, let, call, if, match, …`.
* **Primitive types**: `i32, i64, f64, bool, str, unit`.
* **Nominal types**: `struct`, `sum`, `alias`.
* **First-class functions**: lambdas, explicit signatures.
* **Control flow**: `if`, `while`, `for`, `match` with exhaustive patterns.
* **Errors (primary)**: Rails error-as-values (`ok/ef/es/eh/ec`) with `bind/map/orf/ors/fold/join`. No `try/catch` in Rails.
* **Arrays**: deterministic length, safe indexing, iteration protocol.
* **First-class docs/comments (core)**: `(c kind chan text [id] [(tags …)])` attaches to the next node; compiler/runtime ignore, tools render; preserved by canonicalization.
* **Error rails (gated)**: enable with `(feat rails)` to use error-as-values (`ok/ef/es/eh/ec`) and combinators (`bind/map/orf/ors/fold/join`). Rails disables `throw/try/catch`.
* **Memory model**: ARC (automatic reference counting), deterministic release; no GC in MVP.
* **Hosted profile**: basic `io.print`, `math.*`, `arr.*`, classic `result<ok,err>` helpers. With Rails, prefer `res<T>` + combinators.
* **Deterministic semantics**: left-to-right eval, no unspecified order.

---

## 🧠 Memory Model (ARC, No GC)

* Immutable values by default; composites use **ARC** (automatic reference counting).
* Deterministic release at last reference drop; destructor timing is specified.
* Cycles unsupported in MVP (avoid by construction; weak refs may arrive in V1).

---

## 📦 Project Layout

Current:

```
llm-ir/
├─ README.md
└─ docs/
   ├─ features.md
   ├─ spec-surface.md
   └─ conformance-pack.md
```

Planned (future):

```
llm-ir/
├─ canon/      # Canonicalizer tool
├─ parser/     # Reader for .pulse source
├─ vm/         # Reference bytecode interpreter
├─ tests/      # Conformance suite (golden/pos/neg/diff)
└─ examples/   # Example LLM-IR programs
```

---

## 🛠️ Getting Started

Until the reference implementation lands, the "getting started" path is documentation-driven:

1. Read `docs/spec-surface.md` to understand the lexical/grammar surface, node inventory, and contracts.
2. Browse `docs/features.md` for the MVP scope and the future dials (what’s intentionally deferred).
3. Review `docs/conformance-pack.md` to see executable-style scenarios that define the expected behavior.

When the implementation is available, this section will include build/run/test commands and examples.

---

## 📚 Modularity of Docs

Start here for a spec-first tour:

* `docs/features.md` — overview of MVP core, profiles (Rails, async, caps), and testing model.
* `docs/spec-surface.md` — grammar, node inventory, ARC memory, diagnostics, canonicalization.
* `docs/conformance-pack.md` — runnable-style scenarios grouped by theme.

Future module splits (to keep docs focused): `docs/async.md`, `docs/os.md`, `docs/errors.md`.

---

## 📖 Example Program

Hello World in LLM-IR:

```
(mod main
(compact core)
(caps io)
(fn entry -> i32 ()
(io.print Hello)
0))
```

---

## 🧪 Conformance Testing

The conformance surface is specified in `docs/conformance-pack.md`:

* **Positive tests** must parse, typecheck, run, and return expected value.
* **Negative tests** must fail with precise diagnostics (e.g., `E_PARSE`, `E_TYPE`).
* **Golden tests** define canonicalization round-trips (input → canonical → expected).
* **Differential tests** will compare VM vs backend traces for determinism.

---

## 🚧 Roadmap

* [ ] MVP spec draft (docs in this repo)
* [ ] Canonicalizer prototype
* [ ] Parser + AST validator
* [ ] Reference VM (single-threaded, ARC)
* [ ] Core test suite (≈40 cases → grow to 100+)
* [ ] Profiles: `(feat rails)` (primary errors), `(feat async)` (tasks/chan/timers/select), `(caps ...)`
* [ ] OS/Proc/GPU primitives: `proc.spawn/kill/wait`, stdio streams, GPU list/mask/affinity
* [ ] Deterministic testing hooks: fake clocks, golden logs, sandbox FS
* [ ] LLVM / WASM backend
* [ ] V1 features: generics, FFI, options, macros/CTE

---

## 🤝 Contributing

This is a spec-first project. Please:

* Keep examples and scenarios in canonical form (see `docs/spec-surface.md`).
* When proposing new nodes or behavior, add/update the relevant section(s) and conformance scenarios.
* No feature without a spec section + at least one test scenario in `docs/conformance-pack.md`.

---

## 📜 License

MIT (spec and tools free to use, modify, and experiment with).

---

## 🔭 Future Modules

Planned documentation splits to keep scope modular:

* `docs/async.md` — async runtime, tasks, channels, timers, select, scheduling.
* `docs/os.md` — processes, stdio streams, filesystem, hashing, GPU.
* `docs/errors.md` — Rails semantics, diagnostics, patterns, migration from classic `result`.

---

## 🔁 End-to-End Streaming Example (sketch)

Zero-indent, token-cheap sketch showing headers and flow:

```
(mod srv
(rails)
(compact core)
(async)
(caps fs proc http json time gpu)
(com doc human "Stream llama.cpp NDJSON to SSE")
(fn route -> res<i32> ()
(let s (sse.open "/chat/stream"))
(let p (proc.spawn "/usr/bin/llama.cpp" (v "--json" "-p" "Hi") (v) "."))
(let out (proc.stdout p))
(let t (timer.after 60000))
select
(on_tick t (lam _ (orsoft (soft "timeout") 0)))
(on_recv out (lam line (pipe line (map _ (lam x (sse.send s x))) ok 0)))
(on_deadline (deadline.ms 30000) (lam _ (orsoft (soft "canceled") 0))))
))
```

Notes:

* Orchestrator reads NDJSON lines from `out`, converts each to an SSE `token` frame via `sse.send`, closes on cancel.
```
