# Order #12 - Correct rust-toolchain pin comments

## Request

Fix security finding S5: .github/workflows/deploy.yaml SHA-pins dtolnay/rust-toolchain but comments the pin as # master. Replace misleading comments with accurate tagged-release/ref comments so future updates are not encouraged toward a moving branch.

## Clarifications

### Outcome
Workflow comments accurately describe the pinned dtolnay/rust-toolchain references.

### Scope
.github/workflows/deploy.yaml comments for dtolnay/rust-toolchain uses in CI and publish jobs.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Low provenance/maintenance risk in CI configuration.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: No dtolnay/rust-toolchain SHA pin in deploy.yaml is labeled # master.
- [ ] AC-002: Each action use remains SHA-pinned and has an accurate, reviewable comment such as a tag or immutable ref identifier.
- [ ] AC-003: Workflow YAML remains valid and no unrelated CI behavior changes are introduced.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

deploy.yaml diff and a YAML/static review; run cargo checks only if workflow-affecting edits warrant it.
