# Measure - Order #10 Redact NotUnicode environment errors

## Context inspected

- `src/env.rs` defines `EnvError` and delegated `VarError` Debug, Display, and `Error::source` to `std::env::VarError`.
- `src/secure_env.rs` calls `get_var` from `get_secure_var` / `get_secure_var_or`, so invalid-Unicode errors propagate through secure-env paths.
- Existing tests used `temp-env`; on Unix, `OsStringExt::from_vec` can create invalid UTF-8 environment values for regression coverage.

## Risk

`std::env::VarError::NotUnicode` owns the raw environment value. Delegating formatting or source chaining can expose secret bytes in logs. The safe behavior is a fixed message/redacted Debug output and no source for this variant.

## Constraints

- No new runtime dependencies.
- Preserve `NotPresent` and IO error behavior.
- Keep secure-env behavior compatible while redacting invalid-Unicode errors.
