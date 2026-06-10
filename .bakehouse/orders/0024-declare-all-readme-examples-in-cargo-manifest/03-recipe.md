# Recipe - Order #24 Declare all README examples in Cargo manifest

## Approach

Implement the requested manifest metadata update in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --test examples_test` — exit 0.
