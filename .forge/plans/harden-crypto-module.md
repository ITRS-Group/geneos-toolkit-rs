# Plan: harden-crypto-module

## Metadata

- Plan: plans/harden-crypto-module.md
- Spec: specs/harden-crypto-module.md (SHA-256: 00cc34dd2af094429e7b1aff3ea85ee0e52ab1cad73cb3d51680a9f16e0a0d51)
- Branch: spec/harden-crypto-module
- Steps: steps/harden-crypto-module/
- Created: 2026-04-08
- Status: not-started

## Scope Manifest

- Scope boundaries: `src/secure_env.rs`, `src/env.rs`
- Files to MODIFY: `src/secure_env.rs`, `src/env.rs`
- Files to CREATE: none (tests are inline modules)
- Resource limits: 4 tasks, 2 files modified
- Specialists: none
- Permissions: read/write source files
- Interactive Regression Checkpoint: not-required
- Interaction Loops: []

## Resource Counters

| Resource | Used | Limit |
|----------|------|-------|
| Tasks | 0/4 | 4 |
| Files created | 0 | 0 |
| Files modified | 0/2 | 2 |

## Phase 0: Preflight

- [x] Spec validated
- [x] Branch created: spec/harden-crypto-module
- [x] Step files generated: 4 tasks

## Phase 1: Failing Tests

- [ ] 1.1: Write failing tests for opaque error messages (assert "decryption failed" instead of detailed messages)
- [ ] 1.2: Write failing tests for key file hardening (duplicate keys, oversize, no-equals, world-readable)
- [ ] 1.3: Write failing test for empty ciphertext handling
- [ ] 1.4: Write failing test for Debug redaction

## Phase 2: Implementation

- [ ] 2.1: Opaque decrypt errors | spec: steps/harden-crypto-module/01-opaque-decrypt-errors.md | files: src/secure_env.rs
- [ ] 2.2: Harden parse_key_file | spec: steps/harden-crypto-module/02-harden-parse-key-file.md | files: src/secure_env.rs
- [ ] 2.3: Redact Debug + salt comment | spec: steps/harden-crypto-module/03-redact-debug-salt-comment.md | files: src/env.rs, src/secure_env.rs

## Phase 3: Validation

- [ ] 3.1: All tests pass (`cargo test --all-features`)
- [ ] 3.2: Clippy clean (`cargo clippy --all-features -- -D warnings`)
- [ ] 3.3: Formatting clean (`cargo fmt --check`)

## Phase 4: Cleanup

- [ ] 4.1: Update existing tests for changed error messages | spec: steps/harden-crypto-module/04-update-and-add-tests.md | files: src/secure_env.rs

## Quorum Log

(empty)

## Failure Log

(empty)

## Commit Log

| Phase | SHA | Message |
|-------|-----|---------|
