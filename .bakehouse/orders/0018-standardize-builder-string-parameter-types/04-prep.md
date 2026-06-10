# Prep - Order #18 Standardize builder string parameter types

## Slice 1

- Classification: public API parameter refactor.
- Files: `src/dataview.rs`, `examples/dataview_with_multiple_headlines.rs`, `examples/iterative_rows.rs`, `README.md`.
- Proof plan: inspect the focused diff, run order-specific validation, then run full regression validation.

## TDD note

This `/bake all` batch applied small review findings together. Separate failing-test output was not captured for every non-behavioral or API-refactor slice; final proof is the focused diff plus fresh validation commands.
