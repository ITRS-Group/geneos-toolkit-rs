# Review - Order #21

Reviewed scoped diff: `tests/examples_test.rs`.

`tests/examples_test.rs` discovers examples from `env!("CARGO_MANIFEST_DIR")/examples` instead of a process-relative path.

Decision: pass.
