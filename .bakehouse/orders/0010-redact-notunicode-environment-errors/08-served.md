# Served - Order #10 Redact NotUnicode environment errors

Fixed the invalid-Unicode environment error leak in `src/env.rs` and added regression coverage in `src/env.rs` / `src/secure_env.rs`.

Proof:

- NotUnicode Display, Debug, and source behavior are redacted/suppressed.
- Invalid UTF-8 environment value tests cover `get_var`, `get_var_or`, `get_secure_var`, and `get_secure_var_or` on Unix.
- `cargo test` passed.
- `cargo test --features secure-env` passed.
- `cargo clippy --all-targets --all-features -- -D warnings` passed.
- Security audit passed with 0 findings.
