# Core language (MVP)

* **Canonical text format** — UTF-8, s-expr lists, fixed arity, single delimiter family; unique pretty-print.
* **Deterministic parse & eval order** — left-to-right, no precedence rules.
* **Modules** — `(mod)`, **imports** `(use)`, **visibility** `(pub)`, one module per file, sorted decls.
* **Identifiers** — dotted lowercase atoms (`a.b.c`), stable tokenization.
* **Literals** — `i:`, `f:`, `s:`, `b:true/false`, `n:nil`; canonical numeric/escape rules.
* **Types (nominal)** — `struct`, `sum`, `alias`, primitives (`i32,i64,f64,bool,str,unit`), arrays `arr<T>`, pointers `ptr<T>` (opaque).
* **Functions** — `(fn name (sig (p …)* -> T) block)`, first-class, lambdas `(lam)`.
* **Blocks & statements** — `(do ...)`, `(let)`, `(set)`, `(ret)`, and bare expression statements (no explicit wrapper).
* **Expressions** — `(call)`, `(get base field)`, `(idx base i)`, `(new type init?)`, `(init (kv field expr)*)`, `(as expr type)`.
* **Operators** — arithmetic/compare/logical: `add,sub,mul,div,mod,eq,ne,lt,le,gt,ge,and,or,not`.
* **Control flow** — `(if)`, `(while)`, `(for var iter block)` via iterator protocol, `(match)` with arms.
* **Patterns** — `p_var, p_lit, p_struct, p_variant`; **exhaustiveness** required for `sum`.
* **Errors** — `(throw)`, `(try … (catch …)+)`, uncaught = deterministic abort.
* **Const & globals** — `(const name T expr)` with const-eval whitelist.
* **Diagnostics** — machine-parsable error codes (`E_PARSE…`), byte spans, node paths.
* **Conformance rules** — round-trip canonicalization, positive/negative tests per node.
* **Std core (hosted profile)**

  * `io.print(str)->unit`
  * `math.*` for ints/floats (trapping overflow in MVP)
  * `arr.new/len/get/set`, iteration protocol
  * `result<ok,err>` sum + helpers (`ok/err/is_ok/unwrap`)

## Runtime & memory (MVP)

* **Single-threaded runtime** with deterministic semantics.
* **Memory model** — **ARC (automatic reference counting)**, deterministic release points, defined destructor timing; no GC in MVP.

### Memory Model (ARC, No GC)

* Immutable values; composites use ARC.
* Deterministic release at last reference drop; destructor timing specified.
* Cycles unsupported in MVP; weak refs possible in V1.

## Canonicalization (MVP)

* One space between atoms, one `\n` per list, sorted `kv`/imports/decls, numeric/string normalization, anonymous params canonicalized.

## First-class docs/comments (core)

* `(c kind chan text [id] [(tags …)])` attaches to the next sibling node in the list.
* Kinds: `d` doc, `k` key, `w` warn, `t` todo, `x` explain, `r` ref.
* Channels: `h` human, `l` LLM, `s` spec, `u` user.
* Compiler/runtime ignore `c` nodes; tools can render them. Canonicalization keeps a `c` immediately preceding a node attached when sorting/moving that node.
* Examples:

  ```
  (com doc human "CLI entrypoint")
  (fn entry -> i32 () ...)
  ```

## Governance & profiles

* **Profiles:** `core` (no IO), `hosted` (IO allowed), `rails` (errors-as-values), `async` (tasks/channels/timers/select).
* **Feature gates:** `(feat name)`; illegal usage → `E_FEATURE` or `E_FEATURE_MISSING:<name>`.
* **Capabilities:** `(caps fs proc gpu http json env time hash)` enable OS/IO/crypto features; missing capability → `E_CAP`.
* **Versioning:** implicit schema v1 for MVP; edition field reserved.

### Compact gating `(cmp ...)`

* Single header `(cmp core | * | [A-Z]+)` at module start.
* Letters documented once: `C,M,A,G,O,D,S,N,F,B`.
* Canonicalizer emits one `(cmp ...)` with sorted letters when multiple are present.

### Rails profile (gated)

Enable with `(feat rails)`. Errors become values with explicit combinators.

* Constructors: `ok v`, `ef s`, `es s`, `eh s`, `ec s` (fast, soft, hard, crash).
* Combinators: `bind`, `map`, `orf`, `ors`, `fold`, `join`.
* Severity order for `join`: `ec > eh > es > ef`.
* When rails is enabled, `throw/try/catch` are disabled (`E_FEATURE_CONFLICT:rails`).

---

### Async profile (gated)

Enable with `(feat async)`; async nodes available, subject to capabilities where noted:

* Tasks: `task.spawn(lam) -> tid`, `task.cancel(tid)`, `task.status(tid)->i32`.
* Channels: `chan.new<T>(i:cap)->chan<T>`, `chan.send(chan<T> v)`, `chan.recv(chan<T>)->res<T>` (rails-friendly).
* Timers & deadlines: `timer.after(i:ms)`, `timer.every(i:ms)`, `deadline.ms(i:ms)`.
* Select: `select(on_recv|on_tick|on_deadline ... [default])` branches.
* Await: `await(expr)` to wait for async events (profiled semantics; rails-friendly returns).

Requires capabilities per usage: `time` for timers/deadlines; `env/proc/gpu/fs/http/json/hash` for specific APIs below.

### OS/process & system features (capability-gated)

* Processes: `proc.spawn`, `proc.kill`, `proc.wait`, stdio: `proc.stdout/stderr/stdin`, streams: `stream.read`, sinks: `sink.write` (caps `proc`).
* Filesystem: `fs.watch` for change notifications (caps `fs`).
* Hashing: `hash.sha256` (caps `hash`).
* GPU: `gpu.list`, `gpu.mask` and affinity (caps `gpu`).
* Env/time: `env.*`, fake/frozen clocks for tests (caps `env`, `time`).

### Framing & Re-emit (edge)

* Inputs from workers: `raw | ndjson` via `proc.stdout`.
* Outputs to clients: `sse` (browser). `ws` planned (future profile).

### Security model

* Capability gating is explicit and module-scoped via `(caps ...)`.
* Using a gated op without the capability yields `E_CAP`.

### Testing & determinism

* Fake clocks and deterministic timer/select semantics for tests.
* Golden logs and structured events.
* Sandbox FS and deterministic scheduling under `(feat async)`.

### Extended surface (V1+)

* **Generics (monomorphized)** — `fn< T …>`, `arr<T>`, generic `struct/sum`, deterministic monomorphization order.
* **Options** — `opt<T> = sum { Some(T), None }` with helpers.
* **Maps & sets** — deterministic iteration; profile-tunable hashing.
* **String APIs** — code-point iteration, slicing by grapheme (profiled), conversions.
* **FFI (C ABI)** — `(extern c (fn …))`, POD marshalling, strings as `(ptr<u8>,len)`.
* **Concurrency** — gated by `(feat async)`: tasks, `chan<T>`, `send/recv`, `select`; **HB** edges defined.
* **Atomics & sync** — `fence`, `cas`, progress guarantees (lock-free/wait-free optional).
* **Effects & resources** — effect annotations *or* capability types (design dial).
* **Overflow policy profiles** — `trap` (default) vs `wrap` vs `saturate`, or suffixed ops (`addw/adds`).
* **Metaprogramming (pick one)**

  * **Hygienic macros** `(defmac …)` with gensyms; or
  * **Compile-time eval** `(cte expr)` with totality constraints.
* **Packages** — package metadata, resolution, lockfile, reproducible builds.
* **Reflection** — type/field introspection under a gated profile.
* **Debug/trace** — stable event IDs, stepping hooks, source maps to pretty-printed view.
* **Security** — capabilities for IO/FFI/concurrency; sandboxes for hosted runtimes.
* **Testing DSL** — golden output checks, VM vs backend differential harness baked in.

---

### Spec dials (explicit decisions you’ll lock)

* Memory: **RC vs GC**, destructor semantics.
* Integers: **trap/wrap/saturate** policy & whether selectable per profile/op.
* Strings: **byte-indexed MVP**, upgrade path to Unicode-aware APIs.
* Meta: **macros vs CTE** for V1.
* Generics: **include in MVP** (harder) or **defer to V1** (simpler pipeline).
* Concurrency: stick to **single-threaded MVP**, gate the rest.

---

### Node checklist (MVP)

`mod, use, pub, feat, c, tags, type, struct, sum, alias, const, fn, sig, p, do, let, set, ret, expr, if, while, for, match, arm, call, lam, var, arg, tmp, get, idx, new, init, kv, as, add, sub, mul, div, mod, eq, ne, lt, le, gt, ge, and, or, not, throw, try, catch, p_var, p_lit, p_struct, p_variant, i:, f:, s:, b:, n:nil`

---

## See also

* [Spec surface](./spec-surface.md)
* [Conformance pack](./conformance-pack.md)
* Planned splits: `async.md`, `os.md`, `errors.md`
* Planned: `streaming.md`
