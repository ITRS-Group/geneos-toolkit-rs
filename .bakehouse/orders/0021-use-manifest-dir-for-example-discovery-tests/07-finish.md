# Finish - Order #21 Use manifest dir for example discovery tests

## Outcome

`tests/examples_test.rs` discovers examples from `env!("CARGO_MANIFEST_DIR")/examples` instead of a process-relative path.

## Validation evidence

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.

## Follow-up

No order-specific follow-up recorded.
