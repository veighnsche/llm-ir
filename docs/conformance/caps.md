# Capabilities (negative)

59. Missing caps → E_CAP
```
(mod m
(compact core)
(fn main -> str ()
(hash.sha256 "abc"))
```

Expect: E_CAP on hash.sha256.
