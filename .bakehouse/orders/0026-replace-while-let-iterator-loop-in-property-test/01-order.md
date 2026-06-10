# Order #26 - Replace while-let iterator loop in property test

## Request

Fix clippy finding: src/dataview.rs uses while let Some(c) = chars.next() in a property test where a for loop over the iterator is clearer and clippy-clean.

## Clarifications

### Outcome
The property test uses idiomatic iteration without changing comma-count logic.

### Scope
src/dataview.rs property test that counts unescaped commas.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Test-only clippy cleanup; preserve escaping logic exactly.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: The while-let-on-iterator loop is replaced with an idiomatic for loop or equivalent clippy-clean structure.
- [ ] AC-002: The raw comma counting behavior remains unchanged.
- [ ] AC-003: cargo clippy --all-targets --all-features -- -D warnings passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Test diff plus clippy output.
