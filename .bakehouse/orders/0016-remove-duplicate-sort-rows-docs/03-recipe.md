# Recipe - Order #16 Remove duplicate sort_rows docs

## Approach

Implement the requested rustdoc cleanup in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --doc` — exit 0.
