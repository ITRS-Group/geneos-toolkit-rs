# Order #19 - Avoid per-cell allocations while rendering

## Request

Fix code quality finding 6: write_data_rows performs values.get(&(row.to_string(), col.to_string())) in the render hot loop, allocating two Strings for every cell. Rework lookup/storage to avoid per-cell allocation while preserving output.

## Clarifications

### Outcome
Dataview Display renders cells without cloning row and column names per lookup.

### Scope
src/dataview.rs internal value storage and rendering logic, plus tests for output stability.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Internal representation change can affect ordering or lookup behavior; keep public behavior stable.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Rendering no longer allocates new row/column Strings for each cell lookup.
- [ ] AC-002: Existing dataview output tests and property tests still pass.
- [ ] AC-003: Public API behavior and output ordering remain unchanged.
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

Source diff showing allocation-free lookup path plus test/clippy output.
