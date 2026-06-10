# Prep - Order #17 Return borrowed str from accessors

## Slice 1

- Classification: public API ergonomics refactor.
- Files: `src/dataview.rs`.
- Proof plan: inspect the focused diff, run order-specific validation, then run full regression validation.

## TDD note

This `/bake all` batch applied small review findings together. Separate failing-test output was not captured for every non-behavioral or API-refactor slice; final proof is the focused diff plus fresh validation commands.
