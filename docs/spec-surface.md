# 1) Encoding & Lexical Surface

* **Encoding:** UTF-8 only. Newline = `\n`. No tabs.
* **Tokens (MVP):** `()` `:` `,` `.` `=` `->` `?` `*` `<` `>` `"`. (V1 may add `@` for metadata nodes.)
* **Atoms:**

  * identifiers: `[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)*` (dotted paths)
  * type atoms: same as identifiers, plus arity suffixes (e.g., `arr<i32>`)
  * literals: `i:<int>` `f:<float>` `s:<utf8>` `b:true|false` `n:nil`
* **Whitespace:** single spaces between atoms; one `\n` after each list. No comments. (Metadata nodes are structured and, in V1+, may use `@`.)

# 2) File & Module Surface

* **One module per file.** Top form: `(mod <name> <decl>...)`
* **Deterministic order:** declarations sorted lexicographically by tag, then name.
* **Imports:** `(use <pkg.path> (sym <local>=<remote>)...)` or `(use <pkg.path> (all))`
* **Exports/visibility:** `(pub <decl>)` wrapper for public; private by default.
* **Feature gates:** `(feat name)` must appear at module start (before any non-comment declaration). Violations → `E_FEATURE_MISSING:<name>` for missing gates; `E_FEATURE_CONFLICT:<name>` when mutually exclusive with enabled profile behavior.
* **Compact gating:** `(cmp core|*|[A-Z]+)` appears at module start. Multiple `(cmp ...)` forms MUST be merged by the canonicalizer into one with sorted letters; comments attached to any merged header travel to the resulting single header. Letter set documented once: `C,M,A,G,O,D,S,N,F,B`.

# 3) Grammar Surface (EBNF-ish, all lists are s-exprs)

```
program     := module
module      := (mod ident decl*)
decl        := comment* (type_decl | fn_decl | const_decl | use_decl | pub_decl | feat_decl | caps_decl | cmp_decl)
type_decl   := (type ident type_form)
type_form   := (struct field*) | (sum variant*) | (alias type)
field       := comment* (f ident type)
variant     := comment* (v ident type?)
fn_decl     := (fn ident sig block)
sig         := (sig param* -> type)
param       := (p ident type) | (p type)           ; positional or named (canonicalized)
const_decl  := (const ident type expr)
use_decl    := (use path importset)
importset   := (all) | (sym mapping+)
mapping     := ident | (ident=ident)
pub_decl    := (pub decl)
feat_decl   := (feat ident)
caps_decl   := (caps ident*)
cmp_decl    := (cmp ident) | (cmp "*") | (cmp letters)
letters     := s:<[A-Z]+>                              ; compact gating letters, sorted canonically

block       := (do stmt*)
stmt        := comment* (let | set | ret | expr | if | while | for | match | throw | try)
let         := (let ident expr)
set         := (set lvalue expr)
if          := (if expr block block?)               ; `else` optional
while       := (while expr block)
for         := (for ident expr block)
match       := (match expr arm+)
arm         := comment* (arm pat block)
throw       := (throw expr)
catch       := (catch pat block)
try         := (try block catch+)                    ; gated by profiles (see Rails)

expr        := call | lit | var | lambda | field_get | index | make | cast | op
call        := (call target expr*)
lambda      := (lam sig block)
field_get   := (get expr ident)
index       := (idx expr expr)
make        := (new type init?)
init        := (init kv*)
kv          := comment* (kv ident expr)
cast        := (as expr type)
op          := (add a b) | (sub a b) | (mul a b) | (div a b) | (mod a b)
             | (eq a b) | (lt a b) | (gt a b) | (le a b) | (ge a b) | (ne a b)
             | (and a b) | (or a b) | (not a)
var         := (var ident) | (arg n) | (tmp n)
lit         := i:* | f:* | s:* | b:* | n:nil

pat         := (p_var ident) | (p_lit lit) | (p_struct type (kv ident pat)*) | (p_variant type ident pat?)
type        := prim | (arr<type>) | (ptr<type>) | (res<type>) | (result<ok,err>) | (fn<args->ret>) | (struct ...) | (sum ...)
prim        := i32|i64|f64|bool|str|unit
```

*(Exact EBNF in spec, but this shows node inventory and arities.)*

Doc/comments (first-class, applies to next sibling):

```
comment     := (c kind chan text id? tags?)
kind        := d | k | w | t | x | r           ; doc, key, warn, todo, explain, ref
chan        := h | l | s | u                   ; human, LLM, spec, user
text        := s:<utf8-escaped>                ; minimal escapes: \n \r \t \\ \" \:
id          := i:<u32> | s:<a.z0-9->
tags        := (tags ident*)
```

Placement: a `comment` may appear immediately before any node inside a list (decl list, stmt list, fields, variants, arms, kv). It semantically annotates the next sibling node. A dangling `comment` at the end of a list is invalid (`E_COMMENT_DANGLING`). Unknown kind/channel atoms → `E_COMMENT_KIND`/`E_COMMENT_CHAN`.

# 4) Node Inventory (MVP, with **fixed arity**)

* **Modules & visibility:** `mod(ident, decl*), pub(decl), use(path, importset), feat(name)`
* **Types:** `type(name, form)`, `struct(field*), sum(variant*), alias(type)`, `f(name,type)`, `v(name,type?)`
* **Functions:** `fn(name, sig, block)`, `sig(param*, ret-type)`, `p(name?, type)`
* **Control:** `if(test, then, else?)`, `for(var, iter, block)`, `while(test, block)`, `match(expr, arm+)`, `arm(pat, block)`
* **Blocks/stmt:** `do(stmt*)`, `ret(expr?)`, `let(name, expr)`, `set(lvalue, expr)`
* **Errors:** `throw(expr)`, `try(block, catch+)`, `catch(pat, block)`
* **Expr core:** `call(target, args*)`, `lam(sig, block)`, `get(base, field)`, `idx(base, index)`, `new(type, init?)`, `init(kv*)`, `kv(name, expr)`, `as(expr, type)`
* **Ops:** `add/sub/mul/div/mod/eq/ne/lt/le/gt/ge/and/or/not` (all with fixed arity)
* **Vars & lits:** `var(name)`, `arg(n)`, `tmp(n)`, `i:/f:/s:/b:/n:nil`
* **Patterns:** `p_var(name)`, `p_lit(lit)`, `p_struct(type, kv*)`, `p_variant(type, name, sub?)`
* **Docs/comments:** `c(kind, chan, text, id?, tags?)`, `tags(name*)`

# 5) Static Semantics Surface

* **Name resolution:** lexical, module-scoped; imports create immutable bindings; no shadowing of types by values.
* **Types:** nominal for `struct/sum`, monomorphized generics (V1); arrays `arr<T>` are homogenous, fixed-length if constructed with `new(arr<T>, init)` where `init` arity fixes length.
* **Type inference:** local (expression-level) only in MVP; no global HM unification.
* **Lvalues:** `var`, `get`, `idx` only.
* **Effects model (MVP):** no side-effect typing—IO is via stdlib calls; `throw/try` is a dynamic effect without static tracking.
* **Exhaustiveness:** `match` must be exhaustive over `sum` (static check).
* **Const eval:** `const` requires evaluable `expr` with no runtime ops (list of allowed nodes specified).

# 6) Dynamic Semantics Surface

* **Evaluation order:** left-to-right, call arguments evaluated before call; `let` binds result; `set` writes atomically to lvalue.
* **Short-circuiting:** `and/or` short-circuit; `not` unary.
* **Control:** `if` chooses by `bool`; `for` desugars to iterator protocol calls (`iter.next`) defined by std.
* **Match:** structural match; on `sum` tags dispatch by variant; on `struct` by field names.
* **Error model:** `throw` unwinds to nearest `try`; unhandled throw aborts with `ABORT:UNCAUGHT`.
* **Numbers:** integers wrap or trap? **Spec surface must decide.** (MVP: trap on overflow unless `addw` etc. are used; V1: selectable profile.)
*MVP decision:* trap on overflow. V1 may introduce selectable profiles (trap/wrap/saturate) or suffixed ops.
* **Strings:** UTF-8, length in bytes; indexing by byte offset (V1 may add code-point indexing).
* **Determinism:** no unspecified evaluation; hash maps in core std have deterministic iteration (salted hashing left for V1).

# 7) Memory & Concurrency Surface

* **MVP memory: ARC (automatic reference counting)** with deterministic release points.

  * object identity rules
  * lifetime of temporaries
  * destructor timing (drop order within a block is last-use to first-use)
  * no tracing GC in MVP; cycles must be avoided by construction or weak refs in V1.
* **Data races:** single-threaded MVP.
* **V1 concurrency:** `spawn`, `chan<T>`, `send/recv` define happens-before; no shared mutable data without `atom<T>` with `load/store/xchg` primitives and a **sequentially consistent** default.

# 8) Canonicalization Surface (format = contract)

* **Parentheses & spaces:** exactly one space between atoms; one newline after each list; no trailing spaces.
* **Ordering:** fields and kv pairs sorted by field name; imports sorted; decls sorted.
* **Docs/comments travel with their sibling:** when sorting decls/fields/imports, a `c(...)` immediately preceding a node remains attached to that node and moves with it.
* **Sorting key isolation:** sort keys are computed on the annotated node only; attached comments never affect ordering keys.
* **Compact gating merge:** multiple `(cmp ...)` headers are merged into a single `(cmp ...)` with sorted `letters` (or `core`/`*`), preserving the first header's attached comments.
* **Names:** anonymous params become `(p type)`; formatter rewrites to positional or names per rule.
* **Numeric:** floats normalized (`-0` → `0`, canonical exponent).
* **Strings:** minimal escaping set is `\n \r \t \\ \" \:` and non-printable bytes. Two spellings are allowed for string atoms: `s:Token_Friendly` (unquoted) and `s:"..."` (quoted). The canonicalizer chooses the shortest valid spelling; quoted is used when spaces/escapes are present.

# 9) Diagnostics Surface (machine-parsable)

* **Error code set:** `E_PARSE`, `E_ARITY`, `E_NAME`, `E_TYPE`, `E_LVALUE`, `E_EXHAUST`, `E_OVERFLOW`, `E_CONST`, `E_PRIV`, `E_IMPORT`, `E_UNSAFE`, `E_RUNTIME`, `E_UNCAUGHT`, `E_FEATURE`, `E_FEATURE_MISSING:<name>`, `E_FEATURE_CONFLICT:<name>`, `E_CAP`, `E_COMMENT_DANGLING`, `E_COMMENT_KIND`, `E_COMMENT_CHAN`.
* **Span model:** byte offsets `[start,end)` relative to file; node path trail `(mod/fn[name]/block[idx]/…)`.
* **Repair hints:** compiler MAY emit a minimal edit script (insert/move/replace) for `E_PARSE/E_ARITY`.

# 10) Standard Core Contracts (MVP)

* **Types:** `unit, bool, str, i32, i64, f64, arr<T>`
* **IO:** `io.print(str) -> unit`
* **Math:** `math.add/sub/mul/div/mod` on ints/floats with trapping semantics defined.
* **Array:** `arr.new<T>(i:len, init:fn<i32->T>) -> arr<T>`, `arr.len(arr<T>)->i32`, `arr.get/set`
* **Result (classic, optional):** `result<ok,err>` is a `sum { Ok(ok), Err(err) }` plus helpers `ok/err/is_ok/unwrap` (unwrap traps on `Err`). See also the Rails profile for `res<T>` and combinators.
* **Option (V1):** `opt<T>` = `sum { Some(T), None }`.

# 11) Rails Profile (gated)

Enable with `(feat rails)`. When enabled:

* Nodes enabled: `ok, ef, es, eh, ec, bind, map, orf, ors, fold, join`.
* Nodes disabled: `throw, try, catch` are illegal within this profile (`E_FEATURE_CONFLICT:rails`).
* Type: `res<T>` is interpreted as a sum with severities: `Ok(T) | Ef(EfP) | Es(EsP) | Eh(EhP) | Ec(EcP)` where payload types `EfP/EsP/EhP/EcP` are opaque in MVP (examples use strings).
* Using any rails node without `(feat rails)` → `E_FEATURE_MISSING:rails`.
* Severity lattice: `ec > eh > es > ef > ok`.

Semantics:

* `(ok v)` → `res<T>` ok.
* `(ef s)|(es s)|(eh s)|(ec s)` → `res<T>` with string payload `s` and severities Fast/Soft/Hard/Crash.
* `(bind r lam)` → if `r` is ok, call `lam` on value; else propagate error unchanged.
* `(map r lam)` → if `r` is ok, return `(ok (call lam v))`; else propagate error unchanged.
* `(orf r v)` → if `r` is `ef`, heal to `(ok v)`; else propagate.
* `(ors r v)` → if `r` is `es`, heal to `(ok v)`; else propagate.
* `(fold r fo ff fs fh fc)` → total handler: if `r=ok(v)` return `(call fo v)`; if `ef(e)` return `(call ff e)`; `es(e)` → `(call fs e)`; `eh(e)` → `(call fh e)`; `ec(e)` → `(call fc e)`. Returns a non-res value `U`.
* `(join a b lam)` → if both ok, call `lam` with both values; otherwise, return the error of highest severity among `a` and `b` (severity order `ec>eh>es>ef`). Ties pick the left operand; implementations MAY concatenate messages.
* No up/down-grading nodes in MVP (e.g., `up_es_to_eh`); such nodes may be added later.

Typing sketches:

* `ok: T -> res<T>`
* `ef/es/eh/ec: EfP/EsP/EhP/EcP -> res<T>`    ; payload types opaque; examples use strings
* `bind: res<A> × (fn<A->res<B>>) -> res<B>`
* `map: res<A> × (fn<A->B>) -> res<B>`
* `orf/ors: res<A> × A -> res<A>`
* `fold: res<A> × (fn<A->U>) × (fn<EfP->U>) × (fn<EsP->U>) × (fn<EhP->U>) × (fn<EcP->U>) -> U`
* `join: res<A> × res<B> × (fn<A,B->res<C>>) -> res<C>`

Canonicalization: rails nodes follow normal spacing/ordering rules.

# 11A) Async Profile (gated)

Enable with `(feat async)`.

Nodes (all fixed arity; return types may be profiled):

* Tasks: `task.spawn(lam)->tid`, `task.cancel(tid)->unit`, `task.status(tid)->i32`.
* Channels: `chan.new<T>(i:cap)->chan<T>`, `chan.send(chan<T> v)->res<unit>`, `chan.recv(chan<T>)->res<T>`.
* Timers: `timer.after(i:ms)->tick`, `timer.every(i:ms)->tick`, `deadline.ms(i:ms)->deadline`.
* Select: `select(case+ [default]) -> res<any>` where `case := on_recv(chan<T> lam) | on_tick(tick lam) | on_deadline(deadline lam)`.
* Await: `await(expr)->res<any>`

Determinism:

* Select tie-break rule: first-ready branch in source order wins deterministically.
* Scheduling is single-threaded cooperative in MVP.
* Status codes: `task.status` returns `0=running, 1=done, 2=canceled`.
* Cancellation: `task.cancel` causes blocked ops to complete within a deadline as a soft rails error (e.g., `es s:"canceled"`).
* Await: `await(x)` is only defined for async results and returns `res<T>` (rails), never throws.

Capabilities: timers and deadlines require `time` capability.

# 11B) System Capabilities (capability-gated)

Enable with `(caps ...)` at module start. Missing capability for any op → `E_CAP`.

Nodes (examples):

* Processes: `proc.spawn(cmd argv)->pid`, `proc.kill(pid sig)->unit`, `proc.wait(pid)->i32`, `proc.stdout(pid)->stream`, `proc.stderr(pid)->stream`, `proc.stdin(pid)->sink`.
* Streams: `stream.read(stream i:len)->res<str>`, `sink.write(sink str)->res<i32>`.
* Filesystem: `fs.watch(path)->watch`.
* Hashing: `hash.sha256(str)->str` (hex digest).
* GPU: `gpu.list()->arr<str>`, `gpu.mask(arr<str>)->unit`.
* Streaming (edge, requires `http`): `sse.open(str route)->res<sse>`, `sse.send(sse str)->res<unit>`, `sse.close(sse)->res<unit>`.

Capabilities required:

* `proc` for process ops; `fs` for fs ops; `hash` for hashing; `gpu` for GPU ops; `env` and `time` as appropriate.

# 12) FFI Surface (V1)

* **Foreign decls:** `(extern c (fn name sig abi:c))`
* **Marshalling:** only POD structs & fixed arrays; strings are `(ptr<u8>, len)` pairs.

# 13) Metaprogramming Surface (choose one)

* **MVP none**, or **hygienic macros**: `(defmac name (p*) (template...))` with gensyms; expansion defined before typecheck; must produce canonical nodes.
* **OR** compile-time eval: `(cte expr)` reduces if all inputs const; otherwise `E_CONST`.

# 14) Versioning & Profiles

* **Schema version:** file header is implicit; `mod` carries `(@v i:1)` metadata in V1+.
* **Profiles:** `core` (no FFI, single-threaded), `hosted` (IO allowed), `concurrent` (chan/atomics).
* **Feature gates:** `(feat name)` top-level enables gated nodes; illegal use → `E_FEATURE`.

# 15) Conformance Surface

* **Gold tests:** 1:1 mapping between each node rule and tests (positive & negative).
* **Round-trip:** `canonicalize(parse(x)) == x` for all valid programs.
* **Differential:** VM vs LLVM/WASM backends produce identical traces for a corpus.

---

## Minimal MVP Node Set (checklist)

`mod, use, pub, feat, type, struct, sum, alias, fn, sig, p, do, let, set, ret, expr, if, while, for, match, arm, call, lam, var, arg, tmp, get, idx, new, init, kv, as, add, sub, mul, div, mod, eq, ne, lt, le, gt, ge, and, or, not, throw, try, catch, c, tags, i:, f:, s:, b:, n:nil, p_var, p_lit, p_struct, p_variant`

---

## Human View Contract (guidelines)

* At module start: one `(c d h ...)` and optional `(c k h ...)`.
* For each public decl: a preceding `(c d h ...)` line documenting Purpose / Inputs / Outputs / Errors(rails) in one short line each.
* All human prose must use `(c ...)`—no free-form comments or doc trivia in code.

## See also

* Planned split docs: `async.md`, `os.md`, `errors.md`, `streaming.md`.

## What you still need to decide (design dials)

* **Overflow policy** (trap vs wrap vs saturate; per-profile?).
* **GC vs RC** (and destructor semantics).
* **Macros vs CTE** (one for MVP).
* **String indexing** (bytes only in MVP?).
* **Generics in MVP or V1** (monomorphize later is simpler).