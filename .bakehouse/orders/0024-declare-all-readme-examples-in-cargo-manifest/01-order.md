# Order #24 - Declare all README examples in Cargo manifest

## Request

Fix code quality smaller item: README lists runnable examples that are auto-discovered but lack explicit [[example]] entries in Cargo.toml. Add manifest entries so per-example metadata such as required-features can be expressed.

## Clarifications

### Outcome
Every README-listed runnable example has an explicit Cargo.toml [[example]] entry.

### Scope
Cargo.toml [[example]] sections and any README wording that should match the manifest.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Manifest/documentation change; avoid changing example behavior.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Cargo.toml has explicit [[example]] entries for each README-listed example in examples/.
- [ ] AC-002: Existing example names and paths remain unchanged.
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

Cargo.toml diff plus examples_test output.
