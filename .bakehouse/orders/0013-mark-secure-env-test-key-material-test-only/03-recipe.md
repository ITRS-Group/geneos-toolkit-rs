# Recipe - Order #13 Mark secure-env test key material test-only

## Approach

Implement the requested test documentation/comment update in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `cargo test --features secure-env` — exit 0.
