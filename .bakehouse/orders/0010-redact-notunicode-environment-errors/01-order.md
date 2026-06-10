# Order #10 - Redact NotUnicode environment errors

## Request

Fix security finding S2: EnvError::VarError delegates Debug, Display, and source chaining to std::env::VarError. The NotUnicode variant can embed the raw environment value and leak invalid-UTF-8 secret bytes into logs.

## Clarifications

### Outcome
Invalid-Unicode environment variable errors are reported with fixed, non-secret messages.

### Scope
src/env.rs EnvError formatting/source behavior and tests for get_var/get_var_or/get_secure_var_or paths as applicable.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
Medium security risk; narrow trigger but directly involves environment variables that may contain secrets.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: Display for EnvError::VarError(std::env::VarError::NotUnicode(_)) emits a fixed message that does not include the raw value.
- [ ] AC-002: Debug for EnvError::VarError(std::env::VarError::NotUnicode(_)) emits a fixed/redacted representation that does not include the raw value.
- [ ] AC-003: Error::source for the NotUnicode case does not expose std::env::VarError output that includes the raw value.
- [ ] AC-004: Regression tests cover invalid-Unicode environment values on supported platforms and verify no raw bytes appear in formatted errors.
- [ ] AC-005: cargo test, cargo test --features secure-env, and cargo clippy --all-targets --all-features -- -D warnings pass.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

EnvError tests showing redacted NotUnicode Display/Debug/source behavior, plus full test and clippy output.
