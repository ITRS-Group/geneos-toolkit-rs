# Measure - Order #21 Use manifest dir for example discovery tests

## Context

`tests/examples_test.rs` discovers examples from `env!("CARGO_MANIFEST_DIR")/examples` instead of a process-relative path.

Files in scope: `tests/examples_test.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
