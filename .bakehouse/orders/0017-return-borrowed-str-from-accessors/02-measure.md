# Measure - Order #17 Return borrowed str from accessors

## Context

`Dataview::headline` and `Dataview::value` now return `Option<&str>` and tests compare against string slices.

Files in scope: `src/dataview.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
