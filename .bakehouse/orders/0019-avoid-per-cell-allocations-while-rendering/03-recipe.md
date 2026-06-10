# Recipe - Order #19 Avoid per-cell allocations while rendering

## Approach

Implement the requested internal storage/performance refactor in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
