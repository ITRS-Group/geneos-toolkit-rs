# Cook - Order #20 Remove Row Default derive

## Slice 1

- Classification: API invariant cleanup.
- Files changed: `src/dataview.rs`, `README.md`.

## Implementation

`Row` no longer derives `Default`; docs note rows must be created with `Row::new` and explicit names.

## Validation

- `cargo test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.

Full validation also ran for the completed `/bake all` batch:

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.
