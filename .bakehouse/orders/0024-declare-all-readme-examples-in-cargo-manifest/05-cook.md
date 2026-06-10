# Cook - Order #24 Declare all README examples in Cargo manifest

## Slice 1

- Classification: manifest metadata update.
- Files changed: `Cargo.toml`.

## Implementation

`Cargo.toml` now declares explicit `[[example]]` entries for all examples in `examples/`, including README-listed runnable examples.

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
