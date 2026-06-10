# Order #9 - Fix key-file TOCTOU race

## Request

Fix security finding S1: src/secure_env.rs parse_key_file checks key-file metadata with fs::metadata(path) before File::open(path), allowing a path or symlink swap between check and use. Open the key file first, then validate metadata from the opened file descriptor before reading.

## Clarifications

### Outcome
Key-file size and permission checks are bound to the same opened file that is parsed.

### Scope
src/secure_env.rs parse_key_file and its secure-env tests.

### Autonomy Level
Create and later implement in normal Bakehouse phases; this request only creates the order.

### Risk Tolerance
High security risk; affects secret key-file handling. Changes must preserve redacted path/content errors.

### Environment
geneos-toolkit-rs repository; Rust crate with optional secure-env feature.

### Definition of Done
Finding is fixed, documented if user-facing, and verified by the proof listed below.

## Success Criteria

- [ ] AC-001: parse_key_file opens the key file before checking size or world-readability, and uses file.metadata() for those checks.
- [ ] AC-002: Oversize, missing, invalid, and world-readable key-file tests continue to pass, with a regression test or code proof covering descriptor-bound metadata checks where practical.
- [ ] AC-003: Key-file path and key-file contents remain absent from Display and Debug error output.
- [ ] AC-004: cargo test --features secure-env and cargo clippy --all-targets --all-features -- -D warnings pass.

## Constraints

- Keep the crate compatible with the rust-version in Cargo.toml.
- Do not add third-party runtime dependencies unless explicitly approved.
- Preserve the default build with zero third-party runtime dependencies.
- Keep public/user-facing documentation accurate when behavior or signatures change.

## Non-goals

- Do not address the Redis example unbounded INFO read finding (S4); it is intentionally excluded from this batch.
- Do not implement unrelated refactors beyond the named finding.

## Proof Expectations

Source diff in src/secure_env.rs plus secure-env test output and clippy all-targets/all-features output.
