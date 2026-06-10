# Recipe - Order #9 Fix key-file TOCTOU race

## Accepted approach

1. Refactor key-file handling so `parse_key_file` opens the file before any metadata check.
2. Validate maximum size and Unix world-readability using `File::metadata()` from the opened descriptor.
3. Parse from the same opened `File` that was validated.
4. Preserve opaque/open-file error wording and existing redaction behavior.
5. Add a deterministic Unix regression test using an opener hook to replace the path after open and before validation/parsing.

## Acceptance criteria mapping

- AC-001: `parse_key_file` delegates through an open-first path and metadata comes from `file.metadata()`.
- AC-002: existing oversize, missing, invalid, and world-readable tests pass; new regression covers descriptor-bound checks/read.
- AC-003: existing Display/Debug/path redaction tests pass unchanged.
- AC-004: `cargo test --features secure-env` and `cargo clippy --all-targets --all-features -- -D warnings` pass.

## Non-goals

- No changes to Redis example INFO reading.
- No public API or documentation changes expected.
- No dependency additions.
