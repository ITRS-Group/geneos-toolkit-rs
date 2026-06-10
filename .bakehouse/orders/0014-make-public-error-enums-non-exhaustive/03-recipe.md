# Recipe - Order #14 Make public error enums non-exhaustive

## Approach

Implement the requested public API compatibility hardening in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
