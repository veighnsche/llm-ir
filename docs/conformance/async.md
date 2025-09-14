# Async (gated)

49. Spawn task and status/cancel
```
(mod m
(compact core)
(async)
(fn entry -> i32 ()
(let t (task.spawn (lam _ nil)))
(let s0 (task.status t))
(task.cancel t)
(let s1 (task.status t))
0))
```

50. Bounded channel backpressure
```
(mod m
(rails)
(compact core)
(async)
(fn main -> res<i32> ()
(let c (chan.new<i32> 1))
(chan.send c 1)
(chan.send c 2))
```

51. timer.after + select with deadline
```
(mod m
(rails)
(compact core)
(async)
(fn main -> res<i32> ()
(let t (timer.after 10))
(let d (deadline.ms 5))
select
(on_tick t (lam _ ok 1))
(on_deadline d (lam _ ok 0)))
```

52. Async nodes illegal without async (negative)
```
(mod m
(compact core)
(fn main -> i32 ()
(let t (task.spawn (lam _ nil)))
0))
```
