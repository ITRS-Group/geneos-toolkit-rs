# Measure - Order #19 Avoid per-cell allocations while rendering

## Context

Dataview cell storage is now nested by row then column, allowing render lookups by borrowed `&str` without allocating `(String, String)` keys per cell.

Files in scope: `src/dataview.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
