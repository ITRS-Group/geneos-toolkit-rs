# Order #21 - Use manifest dir for example discovery tests

## Request

Fix code quality smaller item: tests/examples_test.rs uses a relative "examples" path. Make example discovery robust by anchoring it to env!("CARGO_MANIFEST_DIR").

## Clarifications

### Outcome
The integration test discovers examples reliably regardless of process working directory.

### Scope
tests/examples_test.rs test_all_examples_compile path handling.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Test-only change.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Example discovery builds the examples path from env!("CARGO_MANIFEST_DIR").
- [ ] AC-002: test_all_examples_compile still compiles/runs the same example set and honors SKIP_RUN.
- [ ] AC-003: cargo test --test examples_test passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Test diff plus examples_test output.
