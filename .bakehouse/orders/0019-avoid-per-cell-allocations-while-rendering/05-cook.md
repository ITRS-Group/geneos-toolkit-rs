# Cook - Order #19 Avoid per-cell allocations while rendering

## Slice 1

- Classification: internal storage/performance refactor.
- Files changed: `src/dataview.rs`.

## Implementation

Dataview cell storage is now nested by row then column, allowing render lookups by borrowed `&str` without allocating `(String, String)` keys per cell.

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
