# Cook - Order #14 Make public error enums non-exhaustive

## Slice 1

- Classification: public API compatibility hardening.
- Files changed: `src/dataview.rs`, `src/env.rs`, `README.md`.

## Implementation

`DataviewError` and `EnvError` are annotated `#[non_exhaustive]`, and README API notes tell downstream users to keep wildcard match arms.

## Validation

- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.

Full validation also ran for the completed `/bake all` batch:

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test --doc` — exit 0.
- `cargo test --test examples_test` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
- Security audit `.bakehouse/security-audit-20260610-044232.md` — PASS, 0 findings.
