# Tests - Order #19

Order-specific command(s):

- `cargo test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.

Full batch validation:

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.
