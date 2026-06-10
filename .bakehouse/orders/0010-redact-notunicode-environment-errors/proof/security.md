# Security - Order #10

Reviewed risk: formatting invalid-Unicode environment errors can leak secret bytes carried by `std::env::VarError::NotUnicode`.

Mitigation checked:

- `Display` no longer includes the raw value.
- `Debug` no longer includes the raw value.
- `Error::source` returns `None` for NotUnicode, so callers cannot reach the inner `std::env::VarError` through source chaining.
- No new dependencies, network calls, filesystem behavior, or command execution were added.

Automated audit:

```sh
security-auditor/scripts/review-security.sh src/env.rs src/secure_env.rs src/dataview.rs --strict --force --report-dir .bakehouse/orders/0010-redact-notunicode-environment-errors
```

Result: PASS; 3 files scanned, 0 findings, 10/10 categories covered, audit tool failures 0.

Report: `.bakehouse/orders/0010-redact-notunicode-environment-errors/security-audit-20260610-042916.md`.
