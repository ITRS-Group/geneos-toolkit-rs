# Recipe - Order #21 Use manifest dir for example discovery tests

## Approach

Implement the requested test robustness fix in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --test examples_test` — exit 0.
