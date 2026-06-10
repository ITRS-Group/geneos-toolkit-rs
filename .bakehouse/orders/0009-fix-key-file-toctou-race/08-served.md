# Served - Order #9 Fix key-file TOCTOU race

Fixed the key-file TOCTOU race in `src/secure_env.rs` by opening the file before validation, checking size and Unix world-readability with `file.metadata()`, and parsing from the same opened descriptor.

Proof:

- New Unix regression test covers descriptor-bound checks/read after path replacement.
- `cargo test --features secure-env` passed.
- `cargo test` passed.
- `cargo fmt --check` passed.
- `cargo clippy --lib --all-features -- -D warnings` passed.
- Security audit of `src/secure_env.rs` passed with 0 findings.

Caveat: `cargo clippy --all-targets --all-features -- -D warnings` still fails on unrelated pre-existing `src/dataview.rs` test findings tracked by orders #25 and #26; this was waived for order #9 to keep scope limited to the security fix.
