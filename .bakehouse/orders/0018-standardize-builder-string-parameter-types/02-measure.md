# Measure - Order #18 Standardize builder string parameter types

## Context

Builder and `Row` string inputs now consistently use `impl AsRef<str>`; numeric example values were converted to strings and README documents numeric conversion.

Files in scope: `src/dataview.rs`, `examples/dataview_with_multiple_headlines.rs`, `examples/iterative_rows.rs`, `README.md`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
