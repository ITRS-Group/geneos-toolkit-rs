# Taste - Order #11 Align KeyFileFormatError redaction

## Review

Key-file format Display now emits `Key file format error` while Debug remains redacted; secure-env tests assert inner variants separately from safe public formatting.

The implementation was compared to the order acceptance criteria in `01-order.md`; each criterion is satisfied by the focused diff and validation evidence.

## Evidence

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.

Decision: pass.
