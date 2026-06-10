# Tests - Order #26

- `cargo clippy --all-targets --all-features -- -D warnings` — red before fix: exit 101 with `clippy::while_let_on_iterator`; green after fix: exit 0.
- `cargo test test_dataview_column_count_consistency` — exit 0.
