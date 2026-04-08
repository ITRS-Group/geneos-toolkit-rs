# Spec: Harden Crypto Module

## Objective

The `secure-env` crypto module (`src/secure_env.rs`) and its error types (`src/env.rs`) produce opaque, uniform error messages that prevent information leakage and padding oracle attacks. Key file parsing rejects malformed input rather than silently accepting it. Debug formatting redacts crypto-sensitive content.

## Acceptance Criteria

- [ ] All `DecryptionFailed` error paths in `decrypt()` produce a single opaque message `"decryption failed"` — no hex details, no padding specifics, no UTF-8 byte content
- [ ] `parse_key_file` error messages do not echo file paths or file content (key names from arbitrary files)
- [ ] `parse_key_file` rejects duplicate keys with `KeyFileFormatError` instead of silently overwriting
- [ ] `parse_key_file` rejects files larger than 1 KiB before reading line-by-line
- [ ] `parse_key_file` rejects non-empty, non-comment lines that lack `=` with `KeyFileFormatError`
- [ ] On `cfg(unix)`, `parse_key_file` returns an error if the key file is world-readable (mode `o+r`)
- [ ] `decrypt()` returns a clear `DecryptionFailed("empty ciphertext")` for `"+encs+"` (prefix with no payload) instead of falling through to PKCS7 unpadding
- [ ] The redundant `value.len() < 6` check is removed; `is_encrypted()` handles short values
- [ ] Salt field in `parse_key_file` return value has an explanatory comment noting it was used during key derivation and is intentionally unused at decryption time
- [ ] `EnvError` has a manual `Debug` impl that redacts the inner string of `DecryptionFailed` and `KeyFileFormatError` variants
- [ ] All existing tests pass (updated to match new error messages)
- [ ] New tests cover: duplicate key rejection, oversize file rejection, no-`=` line rejection, world-readable permission rejection (unix), empty ciphertext error, `Debug` redaction

## Technical Constraints

- Changes scoped to `src/secure_env.rs` and `src/env.rs` only
- Must remain behind `#[cfg(feature = "secure-env")]` feature gate
- Unix permission check must be `#[cfg(unix)]` guarded
- No new dependencies
- Public API signatures (`parse_key_file`, `decrypt`, `get_secure_var`, `get_secure_var_or`) unchanged

## Design Decisions

- **Single opaque error for all decrypt failures:** All six `DecryptionFailed` paths collapse to `"decryption failed"`. This is the standard defense against padding oracle attacks — distinguishable errors let attackers probe the decryption boundary.
- **File size check before line iteration:** Read metadata (`fs::metadata`) for size check, not the full file. Threshold: 1 KiB.
- **Permission check is error, not warning:** A world-readable key file in a financial services environment is a hard error, not advisory.
- **Redacted Debug:** `DecryptionFailed` prints `DecryptionFailed([REDACTED])`, `KeyFileFormatError` prints `KeyFileFormatError([REDACTED])`. Other variants use default formatting.

## Out of Scope

- **Zeroizing key material** — covered by `add-zeroize-support` (separate task, blocked on this one)
- **Path traversal validation** — L9 notes that `File::open(path)` takes untrusted input. The content echo is fixed here, but path validation itself is deferred: in practice the key file path comes from a trusted configuration source, not user input. Document this assumption.
- **Replacing the crypto primitives** — the AES-256-CBC + PKCS7 scheme is dictated by Geneos compatibility

## Notes

- The salt field (`parse_key_file` line 48) is correct to parse and discard. Salt was consumed during the original PBKDF key derivation step performed by Geneos Gateway. The decryption side only needs key + IV.
- Tests asserting specific error strings (`test_parse_key_file`, `test_decrypt_missing_keyfile`, `test_decrypt_invalid_hex`) will need updates to match opaque messages.
