# Measure - Order #24 Declare all README examples in Cargo manifest

## Context

`Cargo.toml` now declares explicit `[[example]]` entries for all examples in `examples/`, including README-listed runnable examples.

Files in scope: `Cargo.toml`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
