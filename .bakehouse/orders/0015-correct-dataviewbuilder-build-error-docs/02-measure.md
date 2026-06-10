# Measure - Order #15 Correct DataviewBuilder build error docs

## Context

`DataviewBuilder::build` rustdoc now says missing row headers return `DataviewError::MissingRowHeader` instead of panicking.

Files in scope: `src/dataview.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
