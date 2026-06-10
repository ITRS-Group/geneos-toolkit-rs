# Tests - Order #25

- `cargo clippy --all-targets --all-features -- -D warnings` — red before fix: exit 101 with `clippy::octal_escapes`; green after fix: exit 0.
- `cargo test test_null_byte_in_row_name` — exit 0.
