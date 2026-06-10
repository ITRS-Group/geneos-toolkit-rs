# Recipe - Order #20 Remove Row Default derive

## Approach

Implement the requested API invariant cleanup in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
