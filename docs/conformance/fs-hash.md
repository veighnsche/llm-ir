# Filesystem & Hash (capability-gated)

57. fs.watch
```
(mod m
(rails)
(compact core)
(caps fs time)
(async)
(fn main -> res<i32> ()
(let w (fs.watch "/tmp"))
(let d (deadline.ms 5))
select
(on_deadline d (lam _ ok 0)))
```

58. hash.sha256
```
(mod m
(compact core)
(caps hash)
(fn main -> str ()
(hash.sha256 "abc"))
```
