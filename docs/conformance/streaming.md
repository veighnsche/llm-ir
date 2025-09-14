# Streaming (edge, http capability)

61. NDJSON → SSE re-emit
```
(mod m
(rails)
(compact core)
(caps http)
(async)
(fn main -> res<i32> ()
(let s (sse.open "/stream"))
(sse.send s "{\"t\":\"hi\"}")
(sse.send s "{\"t\":\"!\"}")
(sse.close s)
ok 0))
```

62. Cancel closes SSE and kills child
```
(mod m
(rails)
(compact core)
(caps http proc time)
(async)
(fn main -> res<i32> ()
(let s (sse.open "/stream"))
(let p (proc.spawn "/bin/sleep" (v "10") (v) "."))
; client closes SSE → server cancels task, then kills proc within deadline
(task.cancel 0)
(proc.kill p "SIGKILL")
(sse.close s)
ok 0))
```
