# Order #18 - Standardize builder string parameter types

## Request

Fix code quality finding 5: builder APIs mix &str parameters with impl ToString parameters across add_headline, add_value, set_row_header, Row::new, and add_cell. Choose and apply a consistent string-input strategy such as impl Into<String> or AsRef<str>.

## Clarifications

### Outcome
Dataview builder and Row string inputs are consistent and ergonomic.

### Scope
Public builder/Row method signatures, call sites in examples/tests, and docs describing the API.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Public API compatibility and ergonomics risk; requires deliberate design before implementation.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: A single parameter-type strategy is chosen and applied consistently to row/header/headline/cell APIs where appropriate.
- [ ] AC-002: Common call sites with &str and String remain ergonomic; any intentionally removed ToString/numeric behavior is documented.
- [ ] AC-003: README examples, rustdoc examples, and examples/ compile after the signature changes.
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

Design note in the order artifacts, source/docs diff, and test/clippy output.
