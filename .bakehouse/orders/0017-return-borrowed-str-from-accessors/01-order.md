# Order #17 - Return borrowed str from accessors

## Request

Fix code quality finding 4: Dataview::headline() and Dataview::value() return Option<&String>. Change them to return Option<&str> for a more idiomatic and flexible public API.

## Clarifications

### Outcome
Dataview accessors expose borrowed string slices instead of borrowed String values.

### Scope
src/dataview.rs accessor signatures, their implementations, docs, tests, and examples that call them.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Public API signature change; may affect downstream type annotations.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: headline(&self, key: &str) returns Option<&str>.
- [ ] AC-002: value(&self, row: &str, column: &str) returns Option<&str>.
- [ ] AC-003: Tests cover the updated accessor return values without requiring callers to depend on String.
- [ ] AC-004: cargo test and cargo clippy --all-targets --all-features -- -D warnings pass.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Source diff plus test/clippy output.
