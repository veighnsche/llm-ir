# llmir-asyncx

Purpose
- Async profile surface: tasks, channels, timers, select (API traits only).

Public API (draft)
- Types: `TaskId`, `ChanId`, `Tick`, `Deadline`.
- Traits: task, chan, timer, select (no runtime implementation yet).

Status
- All functions return `E_FEATURE_MISSING:async` placeholders.

Pipeline role
- Defines the async profile surface consumed by the VM and higher layers.
- No runtime in this crate; it is purely traits + type aliases for portability.

API surface sketch (initial)
- `TaskId`, `ChanId`, `Tick`, `Deadline` are lightweight newtypes.
- Traits (to be refined):
  - `Tasks`: `spawn(lam) -> TaskId`, `yield_now()`, `cancel(TaskId)`
  - `Chans<T>`: `chan(cap) -> ChanId`, `send(ChanId, T)`, `recv(ChanId) -> res<T>`
  - `Timer`: `sleep(Tick)`, `timeout(Deadline)`
  - `Select`: `select(&[op]) -> idx`

Example flow (pseudo)
```
; spawn a task and wait on a channel
(let c (chan 1)
  (spawn (lam () (send c 42)))
  (ok (recv c)))
```

Dev notes
- Keep traits minimal and transport-agnostic so different runtimes can implement them.
- Return `res<T>` where blocking/waiting can fail or be cancelled.
- Feature flags should allow mocking these traits for unit tests.

Testing
- Start with compile-time tests and trait object wiring.
- Provide a simple mock backend under a test-only feature to simulate task/chan.

TODOs
- Flesh out trait methods and return types.
- Provide mock implementations for tests under features.
