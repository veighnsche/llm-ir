# LLM-IR Specification (RFC-2119)

> This document uses the key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, **MAY** as in RFC-2119.

---

## 0. Domain & Purpose

LLM-IR is a compact, deterministic, **S-expression** language designed so a **small LLM can emit working systems in one shot**, with **ARC (no GC)** memory, **errors-as-values (Rails)**, first-class **structured comments**, and opt-in **profiles** for async and OS integration. The motivating target is a **Home Chatbot Orchestrator** (local multi-GPU workers, process supervision, bounded queues, SSE re-emit).

---

## 1. Kernel Language (Always On)

### 1.1 Canonical Syntax

* **K-S1**: Source is UTF-8 S-exprs. Each list is `(head items…)`.
* **K-S2**: **Zero indent**: no leading spaces/tabs at line start; one space between atoms; blank lines **MUST NOT** appear in canonical form.
* **K-S3**: **Call-free style**: a non-reserved head denotes a function call.
* **K-S4**: Strings:

  * **Bare** without quotes when no spaces/escapes.
  * Otherwise **quoted** `"..."` with minimal escapes. Canonicalizer **MUST** choose the shortest legal spelling.
* **K-S5**: Literals: `true`, `false`, `nil`, integers `1`, floats `2.5`.
* **K-S6**: Operators as heads are permitted: `+ - * / % == != < <= > >= && || !`.

### 1.2 Nodes & Fixed Arity

* **K-N1**: Every head has a **fixed arity/shape** (schema-validated). Extra/missing items → `E_ARITY`.
* **K-N2**: Core structural heads include: `mod`, `fn`, `lam`, `let`, `set`, `if`, `while`, `match`, `arm`, `struct`, `sum`, `alias`.

### 1.3 Types

* **K-T1**: Primitives: `i64`, `f64`, `bool`, `str`, `bytes`, `unit`, `nil`.
* **K-T2**: Data: `struct` fields, `sum` (tagged unions).
* **K-T3**: Collections: `arr<T>`.
* **K-T4**: **Local type inference** is REQUIRED inside expressions; function signatures **MUST** be explicit at module boundaries.

---

## 2. Profiles & Capabilities

### 2.1 Profiles

* **P-R1**: `(rails)` enables **errors as values** and **disables try/catch**; using try/catch under rails → `E_FEATURE_CONFLICT:rails`.
* **P-C1**: `(compact core|all)` enables token-efficient sugar (call-free, pipeline, fn-sugar, positional args, ops, shortest literals). Canonical output **MUST** use one `(compact …)` header per module.
* **P-A1**: `(async)` enables tasks, channels, timers, deadlines, `select`, `await`.

### 2.2 Capabilities

* **CAP-1**: `(caps fs proc env time json hash http gpu …)` declares allowed OS/IO surfaces at **module scope**.
* **CAP-2**: Calling a gated op without its cap **MUST** error `E_CAP`.
* **CAP-3**: `caps` are additive and **MUST** be present before first gated call in the module.

---

## 3. Comments as Data (First-Class)

* **C-1**: Comment node is `(com kind channel "text" id? (tags …)?)`.
* **C-2**: `kind ∈ {doc, key, warn, todo, explain, ref}`.
* **C-3**: `channel ∈ {human, model, spec, user}`.
* **C-4**: **Attach-next rule**: a comment annotates the **immediately following** sibling. A dangling comment at end of a list → `E_COMMENT_DANGLING`.
* **C-5**: Compiler/VM **MUST** ignore comments semantically. Canonicalizer **MUST** move comments **with** their target during sorts.

Example (zero-indent, attach-next):

```
(mod cli
(com doc human "Reverse each word in a .md file")
(fn entry -> res<i32> () 0))
```

---

## 4. Memory Model (ARC, No GC)

* **M-1**: Values are **immutable by default**.
* **M-2**: Composite heap values are managed by **reference counting (ARC)**. On last reference drop, destructors run **deterministically**.
* **M-3**: Cycles are **unsupported** in MVP (no cycle collector).
* **M-4**: `using res body…` **MUST** guarantee timely release (drop) at scope exit, including on cancellation.

---

## 5. Error Model (Rails)

* **R-1**: Rails constructors (all fixed arity): `ok v`, `fast e`, `soft e`, `hard e`, `crash e`.
* **R-2**: Combinators:

  * `bind res lam` (ok→lam; else pass),
  * `map res lam` (ok→transform; else pass),
  * `orfast res v` (fast→ok v),
  * `orsoft res v` (soft→ok v),
  * `fold res on_ok on_fast on_soft on_hard on_crash`,
  * `join a b lam` (both ok→lam; else **higher severity** bubbles).
* **R-3**: Severity order: `crash > hard > soft > fast > ok`.
* **R-4**: **No try/catch** under rails; exceptions profile is **out of scope** for MVP.
* **R-5**: In `fn … -> res<T>`, a non-rails final expression **MUST** be implicitly wrapped as `ok`.

---

## 6. Async Profile (Tasks, Channels, Time)

* **A-1**: Tasks: `task.spawn fn -> res<task>`, `task.cancel task -> res<unit>`, `task.status task -> res<status>`.
* **A-2**: Channels (bounded): `chan.new<T> cap -> res<chan<T>>`, `send/recv`, `try_send/try_recv`.
* **A-3**: Time: `timer.after ms -> res<tick>`, `timer.every ms -> res<tickstream>`, `deadline.ms ms -> res<deadline>`.
* **A-4**: Select:
  `(select (on_recv chan lam) (on_tick tick lam) (on_deadline deadline lam))`
  The **first ready arm wins**; tie-break is **left-to-right** deterministic.
* **A-5**: Await: `await res<T> -> T` (awaits an async result, preserves rails).
* **A-6**: Cancellation: `task.cancel` **SHOULD** cause pending waits to resolve **soft "canceled"** within a bounded deadline.

---

## 7. System & OS Surfaces (Capability-Gated)

* **Proc** (`proc` cap):
  `proc.spawn cmd args env cwd -> res<proc>`
  `proc.kill proc sig -> res<unit>`
  `proc.wait proc -> res<exit>`
  `proc.stdout/stderr -> res<stream>` `proc.stdin -> res<sink>`
  `stream.read stream n -> res<bytes>` `sink.write sink bytes -> res<i64>`
* **FS** (`fs` cap):
  `fs.read path -> res<str>` `fs.write path str -> res<i64>`
  `fs.exists path -> res<bool>` `fs.scan dir -> res<arr<path>>`
  `fs.watch dir -> res<tickstream>` `fs.suffix path sfx -> str`
* **Hash** (`hash` cap): `hash.sha256 bytes|str|path -> res<hex>`
* **GPU** (`gpu` cap): `gpu.list -> res<arr<gpu>>` `gpu.mask i -> res<mask>`
* **Env/Time** (`env`, `time` caps): `env.get/set`, `time.now -> i64`
* **HTTP/SSE** (`http` cap, optional):
  `http.get/post url body? -> res<resp>` `http.code/body -> res<…>`
  `sse.open route -> res<sse>` `sse.send sse str -> res<unit>` `sse.close sse -> res<unit>`

**SYS-1**: All OS calls **MUST** return rails results.
**SYS-2**: **No network egress by default**; `http` cap opt-in.

---

## 8. Standard Capsules (Recommended Namespaces)

* `io`: `io.arg n`, `io.print str`, `io.exit i32`.
* `json`: `json.parse/stringify/get/at`.
* `log`: `log.event k=v …` (stable key=value).
* (Additional capsules MAY be added without altering kernel.)

All capsule calls **MUST** have fixed arity and stable names.

---

## 9. Canonicalization & Formatting

* **CAN-1**: Formatter **MUST** emit zero-indent, one space between atoms, no blank lines.
* **CAN-2**: Comments **MUST** remain adjacent to their target and move with it across sorts.
* **CAN-3**: Strings/numbers **MUST** render in the **shortest legal** form.
* **CAN-4**: A module **MUST** have at most one `(compact …)` header; examples **SHOULD** use `(compact core)` unless specific sugars require `all`.

---

## 10. Diagnostics

* **DIAG-1**: Typing/shape: `E_TYPE`, `E_ARITY`.
* **DIAG-2**: Features: `E_FEATURE_MISSING:<name>`, `E_FEATURE_CONFLICT:<name>`.
* **DIAG-3**: Caps: `E_CAP`.
* **DIAG-4**: Comments: `E_COMMENT_DANGLING`, `E_COMMENT_KIND`, `E_COMMENT_CHANNEL`.
* **DIAG-5**: Diagnostics **MUST** include a stable code and a span/path to the offending node.

---

## 11. Conformance (High-Level)

An implementation **MUST** pass these categories:

1. **Canonical Round-Trip**: `format(parse(x)) == x` for canonical `x` (including comments).
2. **Comments**: attach-next preserved across sorts; dangling → `E_COMMENT_DANGLING`.
3. **Rails**:

   * `bind/map` typing rules;
   * `orfast/orsoft` only heal their grade;
   * `join` chooses higher severity deterministically;
   * `fold` requires 5 arms.
4. **Async**: spawn/cancel/status; bounded channel backpressure; `select` tie-break; deadlines.
5. **Proc/Streams**: spawn+wait, kill, stdout/err streaming; deadline kill escalates `hard` if exceeded.
6. **Caps**: Using gated ops without `(caps …)` → `E_CAP`.
7. **ARC**: Deterministic drop occurs at last reference; `using` enforces close on cancel.

---

## 12. Security & Safety

* **SEC-1**: Default posture is conservative: capabilities deny OS effects unless declared.
* **SEC-2**: No implicit network/disk writes outside declared caps.
* **SEC-3**: Logs **MUST NOT** emit secrets unintentionally (host impl SHOULD provide masking).

---

## 13. Example (Canonical, Zero-Indent)

**CLI: read → transform → write (Rails + Compact + Caps)**

```
(mod cli
(rails)
(compact core)
(caps fs io env)
(com doc human "Reverse each word in a .md file")
(fn entry -> res<i32> ()
(let p (io.arg 1))
(pipe (out_path p)
(fs.read _)
(rev_words _)
(fs.write (fs.suffix $0 "-rev.md") _)
ok 0))
```

**Async + Proc: spawn worker, stream NDJSON, deadline safeguard**

```
(mod svc
(rails)
(compact core)
(async)
(caps proc time log)
(com doc human "Spawn llama.cpp and stream tokens")
(fn run -> res<i32> (prompt)
(let p (proc.spawn "llama.cpp" (v "-p" prompt) (v) "."))
(using p
(select
(on_recv (proc.stdout p) (lam line (log.event "tok" t=line) ok 0))
(on_deadline (deadline.ms 300000) (lam _ (proc.kill p "SIGKILL") (hard "deadline"))))))
```

---

## 14. Non-Goals (MVP)

* No exceptions/try-catch under rails.
* No stop-the-world GC.
* No runtime reflection or ad-hoc macros (beyond `(compact …)` sugar).
* No implicit network egress.

---

## 15. Versioning & Evolution

* Additions **SHOULD** arrive as new capsules or profiles; kernel stability is paramount.
* A new profile **MUST** define: node inventory, gating rules, typing/semantics, diagnostics, and conformance items.
* Backwards-compatible aliasing (e.g., long/short names) is allowed; canonicalizer emits shortest legal spellings.

---

### Appendix A — Reserved Heads (MVP)

`mod fn lam let set if while match arm struct sum alias com rails compact async caps using ok fast soft hard crash bind map orfast orsoft fold join task chan timer deadline select await proc fs env time json hash http gpu io log sse`

*(Implementations MAY accept additional std capsules as long as they are capability-gated and have fixed arities.)*

---

### Appendix B — Minimal Framing for Front-Ends

* Workers MAY emit **NDJSON** lines on `proc.stdout`.
* Orchestrators MAY re-emit **SSE** frames:
  `event: token\ndata:<json>\n\n` via `sse.send`.
* Cancellation: client closes SSE → server `task.cancel` → child `proc.kill` on deadline overrun; errors surface as rails (`soft` cancel, `hard` deadline, `crash` unrecoverable).

---

This specification is **normative** for language behavior and module headers/semantics. Host runtimes and std capsules must adhere to fixed arities, rails returns, capability gating, and ARC determinism described herein.
