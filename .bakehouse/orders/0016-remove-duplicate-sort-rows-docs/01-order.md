# Order #16 - Remove duplicate sort_rows docs

## Request

Fix code quality finding 3: sort_rows has the same rustdoc sentence duplicated. Remove the duplicate line without changing behavior.

## Clarifications

### Outcome
sort_rows rustdoc is not duplicated.

### Scope
src/dataview.rs sort_rows documentation.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Documentation-only.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: sort_rows has one copy of the "Sorts rows in ascending order" sentence.
- [ ] AC-002: cargo test --doc passes or the documentation-only change is otherwise reviewed.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Rustdoc diff plus doc-test output or documented waiver.
