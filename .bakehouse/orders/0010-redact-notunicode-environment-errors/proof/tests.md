# Tests - Order #10

- `cargo test notunicode -- --nocapture` — red: exit 101 before implementation; green: exit 0 after implementation.
- `cargo test --features secure-env not_unicode -- --nocapture` — exit 0.
- `cargo test` — exit 0; 45 unit tests, 2 integration tests, 6 doctests passed; 2 doctests ignored.
- `cargo test --features secure-env` — exit 0; 66 unit tests, 2 integration tests, 6 doctests passed; 2 doctests ignored.
- `cargo fmt --check` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
