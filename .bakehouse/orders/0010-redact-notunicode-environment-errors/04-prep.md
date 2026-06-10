# Prep - Order #10 Redact NotUnicode environment errors

## Slice 1: EnvError redaction

- Classification: security bug fix.
- Files: `src/env.rs`.
- Red proof: add failing NotUnicode formatting/source tests.
- Green proof: implement explicit NotUnicode arms in `Debug`, `Display`, and `Error::source`.

## Slice 2: Secure-env propagation coverage

- Classification: regression tests for feature-gated path.
- Files: `src/secure_env.rs`.
- Proof: active Unix test verifies `get_secure_var` and `get_secure_var_or` propagate the redacted NotUnicode error.

## Slice 3: Validation and clippy blockers

- Classification: cross-order validation support.
- Files: `src/dataview.rs` for orders #25/#26 only.
- Proof: fix the two existing clippy blockers so order #10 can satisfy its full all-targets clippy acceptance command.
