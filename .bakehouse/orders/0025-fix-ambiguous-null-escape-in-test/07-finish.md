# Finish - Order #25 Fix ambiguous null escape in test

The null-byte row-name test now uses an unambiguous Unicode null escape and remains behaviorally equivalent.

Proof:

- `cargo test test_null_byte_in_row_name` passed.
- `cargo clippy --all-targets --all-features -- -D warnings` passed.
