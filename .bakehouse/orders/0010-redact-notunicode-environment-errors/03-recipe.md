# Recipe - Order #10 Redact NotUnicode environment errors

## Approach

1. Match `EnvError::VarError(std::env::VarError::NotUnicode(_))` separately in `Debug`, `Display`, and `Error::source`.
2. Emit fixed text for Display and a fixed redacted representation for Debug.
3. Return `None` from `source()` for the NotUnicode variant so callers cannot format the inner `std::env::VarError`.
4. Add Unix regression tests using invalid UTF-8 `OsString` values for direct `EnvError`, `get_var`, `get_var_or`, `get_secure_var`, and `get_secure_var_or` paths.

## Acceptance mapping

- AC-001: Display fixed message.
- AC-002: Debug fixed/redacted representation.
- AC-003: `source()` suppressed for NotUnicode.
- AC-004: Regression tests verify invalid-Unicode values do not appear in formatted errors.
- AC-005: Full default tests, secure-env tests, and all-targets all-features clippy pass.

## Non-goals preserved

The Redis INFO read finding and unrelated API refactors remain out of scope.
