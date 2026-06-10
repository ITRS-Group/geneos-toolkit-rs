# Prep - Order #9 Fix key-file TOCTOU race

## Slice plan

### Slice 1: Descriptor-bound key-file validation

- Classification: security bug fix.
- Red: add a focused `secure-env` unit test proving validation/parsing use the opened descriptor when the path is replaced after open.
- Green: introduce a private opener helper, validate metadata from `&File`, then parse from that same `File`.
- Refactor: keep parsing logic otherwise unchanged; remove the obsolete `fs::metadata(path)` import/use.

## Expected changed files

- `src/secure_env.rs`
- `.bakehouse/orders/0009-fix-key-file-toctou-race/*` proof artifacts
- `.bakehouse/index.json` / order `state.json` for terminal workflow state

## Validation commands

- `cargo test --features secure-env test_parse_key_file_uses_open_file_descriptor_for_checks_and_read`
- `cargo test --features secure-env`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Security checks

- Confirm no key path or key content is introduced into error strings.
- Confirm open/metadata/read order uses one descriptor.
- Confirm no dependency or public API changes.
