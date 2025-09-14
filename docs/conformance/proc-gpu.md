# Proc & GPU (capability-gated)

53. proc.spawn + wait exit code
```
(mod m
(compact core)
(caps proc)
(fn main -> i32 ()
(let p (proc.spawn "/bin/true" (v) (v) "."))
(proc.wait p))
```

54. proc.kill with signal
```
(mod m
(compact core)
(caps proc)
(fn main -> i32 ()
(let p (proc.spawn "/bin/sleep" (v "10") (v) "."))
(proc.kill p "SIGKILL")
(proc.wait p))
```

55. stdio read/write
```
(mod m
(rails)
(compact core)
(caps proc)
(fn main -> res<i32> ()
(let p (proc.spawn "/bin/cat" (v) (v) "."))
(let w (proc.stdin p))
(let r (proc.stdout p))
(sink.write w "hi")
(map (stream.read r 2) (lam s 0)))
```

56. gpu.list + mask
```
(mod m
(compact core)
(caps gpu)
(fn main -> i32 ()
(let g (gpu.list))
(gpu.mask g)
0))
```
