# Served - Order #21 Use manifest dir for example discovery tests

`tests/examples_test.rs` discovers examples from `env!("CARGO_MANIFEST_DIR")/examples` instead of a process-relative path.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
