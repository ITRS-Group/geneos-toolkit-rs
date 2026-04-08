# Objective: harden-crypto-module

Harden the `secure-env` crypto module to produce opaque error messages, reject malformed key files, and redact crypto details from Debug output.

## Tasks

| # | Task | Depends On | Files |
|---|------|-----------|-------|
| 01 | Opaque decrypt errors | — | src/secure_env.rs |
| 02 | Harden parse_key_file | — | src/secure_env.rs |
| 03 | Redact Debug + salt comment | — | src/env.rs, src/secure_env.rs |
| 04 | Update and add tests | 01, 02, 03 | src/secure_env.rs, src/env.rs |

## Dependency Graph

```
01 ──┐
02 ──┼──> 04
03 ──┘
```

Steps 01–03 are independent and can execute in any order. Step 04 must run last.
