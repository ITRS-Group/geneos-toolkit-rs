# Recipe - Order #15 Correct DataviewBuilder build error docs

## Approach

Implement the requested rustdoc correction in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --doc` — exit 0.
