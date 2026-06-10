# Order #13 - Mark secure-env test key material test-only

## Request

Fix security finding S6: src/secure_env.rs contains functional test key material and ciphertexts. Add an explicit comment making clear the key is for tests only and must never be used in production.

## Clarifications

### Outcome
Test key material is clearly labeled to prevent accidental reuse.

### Scope
Comments near VALID_KEY_FILE_CONTENTS and related secure_env test vectors.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Informational documentation/security hygiene; no runtime behavior change expected.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: A clear comment such as "TEST KEY - never use in production" appears next to the secure-env test key material.
- [ ] AC-002: The comment does not imply the test key protects real data or should be copied into deployments.
- [ ] AC-003: cargo test --features secure-env passes.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Comment diff plus secure-env test output.
