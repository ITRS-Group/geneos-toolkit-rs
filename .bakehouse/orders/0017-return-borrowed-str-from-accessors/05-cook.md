# Cook - Order #17 Return borrowed str from accessors

## Slice 1

- Classification: public API ergonomics refactor.
- Files changed: `src/dataview.rs`.

## Implementation

`Dataview::headline` and `Dataview::value` now return `Option<&str>` and tests compare against string slices.

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
