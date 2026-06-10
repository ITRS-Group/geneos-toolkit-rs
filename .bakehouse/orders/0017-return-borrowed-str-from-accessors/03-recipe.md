# Recipe - Order #17 Return borrowed str from accessors

## Approach

Implement the requested public API ergonomics refactor in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
