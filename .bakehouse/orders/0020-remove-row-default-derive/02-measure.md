# Measure - Order #20 Remove Row Default derive

## Context

`Row` no longer derives `Default`; docs note rows must be created with `Row::new` and explicit names.

Files in scope: `src/dataview.rs`, `README.md`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
