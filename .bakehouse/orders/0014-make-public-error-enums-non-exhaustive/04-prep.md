# Prep - Order #14 Make public error enums non-exhaustive

## Slice 1

- Classification: public API compatibility hardening.
- Files: `src/dataview.rs`, `src/env.rs`, `README.md`.
- Proof plan: inspect the focused diff, run order-specific validation, then run full regression validation.

## TDD note

This `/bake all` batch applied small review findings together. Separate failing-test output was not captured for every non-behavioral or API-refactor slice; final proof is the focused diff plus fresh validation commands.
