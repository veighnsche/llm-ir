# Rails (gated)

40. Enable rails and basic bind/map
```
(mod m
(rails)
(compact core)
(fn plus1 -> i32 (x) (+ x 1))
(fn main -> res<i32> ()
(bind (ok 41) (lam x ok (plus1 x))))
```

41. Propagation (bind) from error
```
(mod m
(rails)
(compact core)
(fn main -> res<i32> ()
(bind (fast "oops") (lam x ok x)))
```

42. orfast/orsoft healing
```
(mod m
(rails)
(compact core)
(fn main -> res<i32> ()
(let a (orfast (fast "e1") 5))
(let b (orsoft (soft "e2") 6))
(join a b (lam x y ok (+ x y))))
```

43. join severity selection
```
(mod m
(rails)
(compact core)
(fn main -> res<i32> ()
(join (hard "bad") (soft "soft") (lam _ _ ok 0)))
```

44. fold total handler
```
(mod m
(rails)
(compact core)
(fn okf -> i32 (x) x)
(fn sf -> i32 (e) 2)
(fn hf -> i32 (e) 3)
(fn cf -> i32 (e) 4)
(fn main -> i32 ()
(fold (hard "boom") okf sf hf cf))
```

45. Rails disables throw/try/catch (negative)
```
(mod m
(rails)
(compact core)
(fn main -> res<i32> ()
; illegal under rails
(try (do (ret (ok i:0))) (catch (p_var e) (do (ret (ok i:0)))))
(ok 0))
```

46. Rails nodes illegal without rails (negative)
```
(mod m
(compact core)
(fn main -> res<i32> () (fast "oops"))
```
