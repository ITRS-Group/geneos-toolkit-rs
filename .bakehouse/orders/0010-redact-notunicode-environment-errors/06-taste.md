# Taste - Order #10 Redact NotUnicode environment errors

## Review against request

- Display for `EnvError::VarError(NotUnicode(_))` now emits fixed text with no raw environment value.
- Debug for the same variant is fixed/redacted.
- `Error::source` returns `None`, preventing callers from formatting the inner `std::env::VarError`.
- Unix regression tests cover direct env helpers and secure-env helper propagation.

## Acceptance criteria

- AC-001: Passed. Display is `Environment variable error: environment variable value is not valid Unicode`.
- AC-002: Passed. Debug is `VarError(NotUnicode([REDACTED]))`.
- AC-003: Passed. `std::error::Error::source(&err)` is `None` for NotUnicode.
- AC-004: Passed. Tests use invalid UTF-8 values containing secret markers and assert formatted output omits them.
- AC-005: Passed. `cargo test`, `cargo test --features secure-env`, and `cargo clippy --all-targets --all-features -- -D warnings` passed.

## Security review

Automated security audit command:

```sh
security-auditor/scripts/review-security.sh src/env.rs src/secure_env.rs src/dataview.rs --strict --force --report-dir .bakehouse/orders/0010-redact-notunicode-environment-errors
```

Result: PASS; 3 files scanned, 0 findings, 10/10 categories covered, dependency audit OK.

Report: `.bakehouse/orders/0010-redact-notunicode-environment-errors/security-audit-20260610-042916.md`.

## Taste decision

Pass. The secret-leak path is redacted and the requested full validation suite passes.
