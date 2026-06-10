# Measure - Order #23 Assert headline count via public API

## Context

The dataview unit test now checks headline count through `headline_order().len()` instead of the private `headlines` field.

Files in scope: `src/dataview.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
