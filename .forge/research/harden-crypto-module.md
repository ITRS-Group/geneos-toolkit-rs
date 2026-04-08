# Research: harden-crypto-module

## Investigation Type
Codebase analysis — security hardening of existing crypto module

## Source
Intake artifact: intake/harden-crypto-module.md

## Findings

### Codebase Analysis

**Module structure:**
- `src/env.rs`: `EnvError` enum (line 5), `is_encrypted()` (line 102), `get_var`/`get_var_or`
- `src/secure_env.rs`: `parse_key_file` (line 9), `decrypt` (line 51), `get_secure_var`/`get_secure_var_or`
- Feature-gated behind `secure-env` Cargo feature

**L2 — Error message info leakage (secure_env.rs):**
- Line 60: hex decode error includes hex library error detail
- Line 65: key hex decode includes detail
- Line 67: iv hex decode includes detail
- Line 74: PKCS7 unpad failure includes padding error specifics
- Line 77: UTF-8 conversion failure includes byte details
- Lines 10-12 in `parse_key_file`: file path echoed in IoError

**L3 — Distinguishable padding oracle paths (secure_env.rs):**
Six distinct `DecryptionFailed` error paths at lines 60, 65, 67, 72, 74, 77 — each with different error string content. An attacker probing encrypted values could distinguish hex errors from padding errors from UTF-8 errors.

Fix: Collapse lines 72, 74, 77 (post-decryption: key/iv length, PKCS7 unpad, UTF-8) into a single opaque `"decryption failed"`. Pre-decryption hex errors (60, 65, 67) can share a second opaque message or the same one.

**L1 — Key file parsing robustness (secure_env.rs `parse_key_file`):**
- Line 27-38: `match` on `split_once('=')` — `None` arm (no `=`) silently skips the line
- No duplicate key detection: second `"key"` line overwrites first silently
- No file size limit: arbitrary file read into memory
- No Unix permission check on key file
- Lines 32-35: unexpected key names echoed in `KeyFileFormatError`

**I1 — Redundant length check (secure_env.rs line 54):**
`value.len() < 6 || !is_encrypted(value)` — the length check is redundant because `is_encrypted()` calls `starts_with("+encs+")` which already fails on strings < 6 chars. Edge case: `"+encs+"` (exactly 6 chars, empty ciphertext after prefix) passes both checks, reaches line 58 `&value[6..]` producing empty string, then hex decode of empty string succeeds producing empty Vec, which then fails at PKCS7 unpadding with a confusing error.

**I2 — Salt parsed but unused (secure_env.rs):**
- Line 48: `parse_key_file` returns `(salt, key, iv)`
- Line 62: caller destructures as `let (_, key_hex, iv_hex)` — salt discarded
- Correct behavior: salt was used during key derivation, not needed for decryption
- Missing explanatory comment

**L4-partial — Debug derive leaks crypto details (env.rs line 5):**
`#[derive(Debug)]` on `EnvError` means `DecryptionFailed(String)` and `KeyFileFormatError(String)` expose their inner strings in panic output / debug formatting.

**L9 — Key file path traversal + content echo (secure_env.rs):**
- Line 10: `File::open(path)` with no path validation
- Lines 32-35: `KeyFileFormatError` includes the unexpected key name from the file, leaking left-hand content of `=`-delimited lines from arbitrary files

### Scope Estimate

**Files affected:** 2 (`src/secure_env.rs`, `src/env.rs`)
**Estimated complexity:** Medium — 10 discrete changes, most are small, but test updates will be significant since many tests assert specific error messages.

### Existing Conventions
- Error wrapping with `map_err` and closures
- Feature-gating with `#[cfg(feature = "secure-env")]`
- `ok_or_else` for Option validation
- Extensive test coverage (12 test functions for secure_env alone)

### Test Impact
Tests that assert specific error strings will need updating:
- `test_parse_key_file` (line 140): asserts `KeyFileFormatError` content
- `test_decrypt_missing_keyfile` (line 377): asserts IoError path content
- `test_decrypt_invalid_hex` (line 388): asserts `DecryptionFailed` content

## Recommendation
Clear enough to spec. All findings are well-scoped to two files with known line locations. No external dependencies or API changes needed.

## Related Work
- `fix-protocol-injection` (done): established escaping patterns in dataview.rs
- `baseline-test-coverage` (done): established test conventions
- `add-zeroize-support`: blocked on this task, will add zeroize to key material after these hardening changes
