# Core Language

## A) Parse & Canonicalization

1. Atoms & spacing (golden round-trip)

```
(mod m
(compact core)
(const a i32 1))
```

2. Ordering of decls (canonicalize sorts)
Input:

```
(mod m
(compact core)
(fn z -> i32 () 0)
(const a i32 1))
```

Expect canonical:

```
(mod m
(compact core)
(const a i32 1)
(fn z -> i32 () 0))
```

3. KV sort in init

```
(new Point (init (kv y 2) (kv x 1)))
```

4. Arity check (negative)

```
(add 1)
```

Expect: E_ARITY on add.

## B) Modules, use, visibility

5. Import specific symbols

```
(mod m
(compact core)
(use lib.math (sym add))
(fn f -> i32 () (add 1 2)))
```

6. Illegal import alias (negative)

```
(use lib (sym x=y=z))
```

7. pub wrapper

```
(pub (fn f -> i32 () 0))
```

## C) Types & name resolution

8. Struct & field access

```
(mod m
(compact core)
(type Point (struct (f x i32) (f y i32)))
(fn f -> i32 ()
(let p (new Point (init (kv x 1) (kv y 2))))
(get p x)))
```

9. Sum + match (exhaustive)

```
(mod m
(compact core)
(type R (sum (v Ok i32) (v Err str)))
(fn f -> i32 (r)
(match r
(arm (p_variant R Ok (p_var v)) v)
(arm (p_variant R Err (p_var _)) 0))) )
```

10. Non-exhaustive match (negative)
Remove the Err arm → E_EXHAUST on match.

11. Alias type

```
(type Id (alias i64))
(const a Id 7)
```

## D) Functions, lambdas, calls

12. First-class lambdas

```
(mod m
(compact core)
(fn apply -> i32 (f x) (f x))
(fn add1 -> i32 (x) (+ x 1))
(fn main -> i32 () (apply add1 41)))
```

13. Unknown symbol (negative)

```
(mod m
(compact core)
(fn f -> i32 () (nope)))
```

## E) Control flow & evaluation order

14. If/else

```
(mod m
(compact core)
(fn f -> i32 (x) (if (> x 0) 1 2)))
```

15. Left-to-right eval (assume io.print returns unit)

```
(mod m
(compact core)
(caps io)
(fn main -> i32 ()
(io.print A)
(io.print B)
0))
```

16. Short-circuit AND

```
(mod m
(compact core)
(caps io)
(fn main -> i32 () (if (&& false (io.print NEVER)) 1 0)))
```

## F) Arrays

17. arr.new + idx + set

```
(mod m
(compact core)
(fn main -> i32 ()
(let a (arr.new<i32> 3 (lam i i)))
(set (idx a 1) 7)
(idx a 1)))
```

18. Out-of-bounds (negative, runtime)
`(idx a 99)` → E_RUNTIME (OOB).

## G) Ops & numerics

19. Integer trap on overflow (policy)

```
(+ 2147483647 1)
```

Expect: E_OVERFLOW under trap policy.

20. Float NaN compare

```
(== f:NaN f:NaN)
```

Expect usually false.

## H) Errors: throw/try (non-rails)

21. Caught error

```
(mod m
(compact core)
(type E (sum (v Oops i32)))
(fn main -> i32 ()
(try
(throw (new E (init (kv Oops 1))))
(catch (p_variant E Oops (p_var _)) 1))) )
```

22. Uncaught (negative)
Same throw without try → abort E_UNCAUGHT.

## I) Const eval

23. Legal const

```
(const N i32 (+ 2 3))
```

24. Illegal const (IO)

```
(const S str (io.print x))
```

Expect: E_CONST.

## J) Patterns

25. Struct pattern

```
(type P (struct (f x i32) (f y i32)))
(fn f -> i32 (p)
(match p
(arm (p_struct P (kv x (p_var a)) (kv y (p_var b))) a)))
```

26. Literal pattern mismatch

```
(match 2
(arm (p_lit 1) 0))
```

Expect E_EXHAUST.

## K) Casting

27. as cast success

```
(as 1 i64)
```

28. Invalid cast (negative)

```
(as hi i32)
```

Expect E_TYPE.

## L) Lvalues & assignment

29. set on var

```
(mod m
(compact core)
(fn main -> i32 ()
(let x 1)
(set (var x) 2)
(var x)))
```

30. Illegal lvalue (negative)

```
(set (+ 1 2) 3)
```

Expect E_LVALUE or E_TYPE.

## M) Visibility & privacy

31. Private type used outside (multi-module) → E_PRIV (sketch).

## N) Diagnostics format

32. Span + node path

```
code=E_PARSE
span=[byteStart,byteEnd)
path=mod/main/fn[f]/block[0]/…
```

## O) Canonicalizer edge cases

33. Numeric normalization: f:1.2300e+00 → f:1.23

34. String escapes: s:line\nbreak minimal escapes; round-trip equals.

## P) Differential & determinism

35. VM vs backend: identical trace.

36. Deterministic iteration: fixed order for arrays.

## Q) Fuzz & repair

37. Structural-repair loop: minimal edit suggestion then re-parse.
