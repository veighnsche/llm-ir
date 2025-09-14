# LLM-IR Cheat Sheet (token-efficient)

## 0. Canon

* Syntax: `( … )` s-exprs, zero indent, 1 space sep.
* Calls: call-free `(fn arg …)`.
* Strings: bare if no space, else `"..."`.
* Bool/nil: `true false nil`.
* Numbers: `1 2.5`.
* Comments: `(com kind chan "text")` attaches to next node.
* Pipeline: `(pipe expr f1 f2 …)`, `_` placeholder.
* Lambda: `(lam x body…)`.
* Args: `$0 $1 …`.
* Ops: `+ - * / % == != < <= > >= && || !`.

## 1. Headers

* `(rails)` → errors as values, no try/catch.
* `(compact core|all)` → callfree + sugar.
* `(async)` → tasks, chan, timers, select, await.
* `(caps fs proc env time json hash http gpu …)` → caps gating; missing = `E_CAP`.

## 2. Types

* Prim: `i64 f64 bool str bytes unit nil`.
* Data: `struct`, `sum`.
* Collections: `arr<T>`.
* Config: schema-checked to struct.

## 3. Memory

* ARC (ref counting). No GC.
* Immutability by default. Deterministic release at last ref drop. No cycles in MVP.
* `using res body` auto closes.

## 4. Errors (Rails)

* Constructors: `ok fast soft hard crash`.
* Combinators: `bind map orfast orsoft fold join`.
* Severity: `crash>hard>soft>fast>ok`.
* Implicit tail-ok in `res<T>` fn.

## 5. Async

* Tasks: `task.spawn fn`, `task.cancel t`, `task.status t`.
* Await: `await res<T> -> T`.
* Chan: `chan.new<T> cap`, `send/recv`, `try_send/try_recv`.
* Timers: `timer.after ms`, `timer.every ms`.
* Deadline: `deadline.ms ms`.
* Select: `(select (on_recv c lam) (on_tick t lam) (on_deadline d lam))`.
* Cancel → `soft "canceled"`.

## 6. OS / Proc / GPU

* Proc: `proc.spawn cmd args env cwd`, `proc.kill`, `proc.wait`.
* Std: `proc.stdout/stderr -> stream`, `proc.stdin -> sink`.
* Streams: `stream.read`, `sink.write`.
* GPU: `gpu.list`, `gpu.mask`.
* Env/time: `env.get/set`, `time.now`.

## 7. FS / Hash / JSON / HTTP / SSE

* FS: `fs.read/write/exists/scan/watch/suffix`.
* Hash: `hash.sha256 str|bytes|path`.
* JSON: `json.parse/stringify/get`.
* HTTP: `http.get/post`, `srv.listen/next/reply` (opt).
* SSE (edge only): `sse.open route`, `sse.send s str`, `sse.close s`.

## 8. Logs & Obs

* Log: `log.event k=v …`.
* Counters/timers optional.

## 9. Diagnostics

* `E_TYPE`, `E_ARITY`, `E_FEATURE_MISSING:<x>`, `E_FEATURE_CONFLICT:<x>`, `E_CAP`, `E_COMMENT_*`.

## 10. Example skeleton

```pulse
(mod m
(rails)
(compact core)
(async)
(caps fs proc env time json hash http gpu)
(com doc human "Spawn worker and stream")
(fn main -> res<i32> ()
(pipe (proc.spawn "llama.cpp" (v "-m" "/m.bin") (v ("GPU" (env.get "GPU_MASK"))) ".")
(proc.stdout _)
ok 0)))
```
