# Order #14 - Make public error enums non-exhaustive

## Request

Fix code quality finding 1: public exhaustive enums DataviewError and EnvError can make future variants semver-breaking. Add #[non_exhaustive] while the crate is still pre-1.0, and update any docs/tests affected by the public API change.

## Clarifications

### Outcome
Public error enums can grow variants without forcing exhaustive downstream matches.

### Scope
src/dataview.rs DataviewError, src/env.rs EnvError, and any API docs/tests that mention matching these enums.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
API compatibility change for downstream exhaustive matches; release notes or migration docs may be needed.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: DataviewError and EnvError are annotated #[non_exhaustive].
- [ ] AC-002: Internal tests and examples compile after the annotations.
- [ ] AC-003: Documentation or release notes mention the source compatibility implication if this lands in a public release.
- [ ] AC-004: cargo test, cargo test --features secure-env, and cargo clippy --all-targets --all-features -- -D warnings pass.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Source diff plus test/clippy output and documentation diff if added.
