# Order #22 - Normalize crate rustdoc fence order

## Request

Fix code quality smaller item: src/lib.rs has a code fence written as ```no_run,rust. Normalize it to ```rust,no_run.

## Clarifications

### Outcome
Crate-level rustdoc code fence attributes follow conventional ordering.

### Scope
src/lib.rs crate-level documentation.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Documentation-only.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: The crate-level code fence uses ```rust,no_run.
- [ ] AC-002: cargo test --doc passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Documentation diff plus doc-test output.
