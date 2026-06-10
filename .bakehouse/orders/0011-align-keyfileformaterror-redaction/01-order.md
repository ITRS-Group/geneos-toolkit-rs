# Order #11 - Align KeyFileFormatError redaction

## Request

Fix security finding S3: EnvError::KeyFileFormatError Debug redacts the inner message but Display prints it verbatim. Current messages are library-generated, but the redaction posture should be consistent and future-safe.

## Clarifications

### Outcome
Key-file format errors have consistent redacted public formatting.

### Scope
src/env.rs EnvError formatting and secure-env tests that assert key-file format messages.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Low security hardening; may affect callers/tests that assert exact key-file format text.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Display for KeyFileFormatError no longer prints arbitrary inner message content.
- [ ] AC-002: Debug for KeyFileFormatError remains redacted.
- [ ] AC-003: Existing key-file parse tests are updated to assert variants or safe messages rather than relying on potentially sensitive Display text.
- [ ] AC-004: User-facing errors remain understandable enough to identify a key-file format problem without leaking file contents.
- [ ] AC-005: cargo test --features secure-env and cargo clippy --all-targets --all-features -- -D warnings pass.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

EnvError formatting tests plus secure-env test output.
