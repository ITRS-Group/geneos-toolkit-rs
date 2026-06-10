# Order #23 - Assert headline count via public API

## Request

Fix code quality smaller item: a dataview unit test asserts directly on the private headlines field instead of using a public accessor. Update the test to use public API such as headline_order() or headline().

## Clarifications

### Outcome
Tests validate behavior through the same public surface users rely on.

### Scope
src/dataview.rs test around the headline count assertion.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Test-only change.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: The test no longer reads dataview.headlines directly for the headline-count assertion.
- [ ] AC-002: The replacement assertion verifies equivalent observable behavior through public accessors.
- [ ] AC-003: cargo test dataview passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Test diff plus relevant cargo test output.
