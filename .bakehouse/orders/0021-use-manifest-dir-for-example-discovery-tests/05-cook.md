# Cook - Order #21 Use manifest dir for example discovery tests

## Slice 1

- Classification: test robustness fix.
- Files changed: `tests/examples_test.rs`.

## Implementation

`tests/examples_test.rs` discovers examples from `env!("CARGO_MANIFEST_DIR")/examples` instead of a process-relative path.

## Validation

- `cargo test --test examples_test` — exit 0.

Full validation also ran for the completed `/bake all` batch:

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.
