# Order #25 - Fix ambiguous null escape in test

## Request

Fix clippy finding: src/dataview.rs has an ambiguous "\0" followed by a digit in a test string around the null-byte row-name test. Use an unambiguous escape such as \u{0} or \x00.

## Clarifications

### Outcome
The null-byte test string is unambiguous and clippy-clean.

### Scope
src/dataview.rs null-byte row-name test.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Test-only clippy cleanup.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: The ambiguous null escape is replaced with an unambiguous null-byte expression.
- [ ] AC-002: The test continues to cover a row name containing a null byte followed by a digit or equivalent adjacent character case.
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
