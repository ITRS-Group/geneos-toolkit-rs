# Order #15 - Correct DataviewBuilder build error docs

## Request

Fix code quality finding 2: DataviewBuilder::build rustdoc says a missing row_header causes a panic, but the implementation returns Err(DataviewError::MissingRowHeader). Correct the documentation.

## Clarifications

### Outcome
build() rustdoc accurately describes error return behavior.

### Scope
src/dataview.rs rustdoc for DataviewBuilder::build.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Documentation-only; ensure examples remain valid rustdoc.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: The build() docs no longer claim a panic for a missing row header.
- [ ] AC-002: The docs state that MissingRowHeader is returned when the row header is absent.
- [ ] AC-003: cargo test --doc passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Rustdoc diff plus doc-test output.
