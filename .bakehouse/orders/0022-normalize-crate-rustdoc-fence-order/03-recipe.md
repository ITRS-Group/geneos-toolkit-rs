# Recipe - Order #22 Normalize crate rustdoc fence order

## Approach

Implement the requested rustdoc formatting cleanup in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --doc` — exit 0.
