# Prep - Order #22 Normalize crate rustdoc fence order

## Slice 1

- Classification: rustdoc formatting cleanup.
- Files: `src/lib.rs`.
- Proof plan: inspect the focused diff, run order-specific validation, then run full regression validation.

## TDD note

This `/bake all` batch applied small review findings together. Separate failing-test output was not captured for every non-behavioral or API-refactor slice; final proof is the focused diff plus fresh validation commands.
