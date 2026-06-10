# Order #20 - Remove Row Default derive

## Request

Fix code quality finding 7: Row derives Default, which creates an empty-named row that only fails later during DataviewBuilder::build. Remove the derive unless a real use case is documented.

## Clarifications

### Outcome
Rows must be created with an explicit name via Row::new.

### Scope
src/dataview.rs Row derives and any tests/docs that rely on Row::default().

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Public API compatibility change if downstream users call Row::default().

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Row no longer implements Default by derive unless an explicit, justified implementation is retained.
- [ ] AC-002: Tests and examples create rows through Row::new or another explicit-name API.
- [ ] AC-003: Documentation reflects that row names are required.
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

Source/docs diff plus test/clippy output.
