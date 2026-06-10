# Taste - Order #9 Fix key-file TOCTOU race

## Review against request

- The implementation opens the key file before metadata checks.
- Size and Unix world-readability checks use `File::metadata()` from the opened descriptor.
- The same opened `File` is moved into `BufReader` and parsed.
- No public API, dependency, or user-facing documentation changes were introduced.

## Acceptance criteria

- AC-001: Passed. `src/secure_env.rs` no longer uses `fs::metadata(path)` in `parse_key_file`; descriptor metadata is checked in `validate_key_file_metadata(&File)`.
- AC-002: Passed. Existing oversize, missing, invalid, and Unix world-readable tests pass under `cargo test --features secure-env`. New Unix regression test `test_parse_key_file_uses_open_file_descriptor_for_checks_and_read` proves checks/read stay bound to the opened descriptor when the path is replaced after open.
- AC-003: Passed. Existing no-path-leak and Debug redaction tests pass; the change preserves opaque `cannot open key file` IO messages and does not add path/content to errors.
- AC-004: Partially passed with waiver. `cargo test --features secure-env` passed. The exact clippy command failed only on unrelated pre-existing `src/dataview.rs` test findings tracked by orders #25 and #26; resolving those here would violate this order's non-goal. Narrow `cargo clippy --lib --all-features -- -D warnings` passed for library code including `secure_env`.

## Commands and results

- `cargo test --features secure-env test_parse_key_file_uses_open_file_descriptor_for_checks_and_read` — exit 0.
- `cargo test --features secure-env` — exit 0.
- `cargo test` — exit 0.
- `cargo fmt --check` — exit 0.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 101 due unrelated orders #25/#26 findings.
- `cargo clippy --lib --all-features -- -D warnings` — exit 0.

## Security review

Automated security audit command:

```sh
security-auditor/scripts/review-security.sh src/secure_env.rs --strict --force --report-dir .bakehouse/orders/0009-fix-key-file-toctou-race
```

Result: PASS; 1 file scanned, 0 findings, 10/10 categories covered, cargo-audit OK.

Report: `.bakehouse/orders/0009-fix-key-file-toctou-race/security-audit-20260610-035353.md`.

## Taste decision

Pass with recorded clippy waiver for the exact all-targets command. The security bug is fixed and validated; the remaining clippy failures are known unrelated orders and should be handled separately.
