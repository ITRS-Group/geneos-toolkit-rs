# Cook - Order #23 Assert headline count via public API

## Slice 1

- Classification: test refactor to public API.
- Files changed: `src/dataview.rs`.

## Implementation

The dataview unit test now checks headline count through `headline_order().len()` instead of the private `headlines` field.

## Validation

- `cargo test dataview` — exit 0.

Full validation also ran for the completed `/bake all` batch:

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.
