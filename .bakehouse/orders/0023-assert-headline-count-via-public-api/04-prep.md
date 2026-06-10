# Prep - Order #23 Assert headline count via public API

## Slice 1

- Classification: test refactor to public API.
- Files: `src/dataview.rs`.
- Proof plan: inspect the focused diff, run order-specific validation, then run full regression validation.

## TDD note

This `/bake all` batch applied small review findings together. Separate failing-test output was not captured for every non-behavioral or API-refactor slice; final proof is the focused diff plus fresh validation commands.
