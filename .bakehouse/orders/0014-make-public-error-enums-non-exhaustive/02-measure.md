# Measure - Order #14 Make public error enums non-exhaustive

## Context

`DataviewError` and `EnvError` are annotated `#[non_exhaustive]`, and README API notes tell downstream users to keep wildcard match arms.

Files in scope: `src/dataview.rs`, `src/env.rs`, `README.md`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
