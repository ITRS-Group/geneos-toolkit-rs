# Harden Crypto Module

## Findings Addressed

### L2: Error Message Information Leakage (LOW)
Crypto error messages include hex decode details, file paths, and padding error specifics.
Normalize all crypto errors to opaque messages for external consumption.

### L3: Distinguishable Padding Oracle Error Paths (LOW)
Padding errors vs UTF-8 errors vs hex errors are distinguishable via different error strings.
Collapse post-decryption error paths (PKCS7 unpad failure and UTF-8 failure) into a single
`"decryption failed"` message.

### L1: Key File Parsing Robustness (LOW)
- Duplicate keys silently overwrite — reject on duplicate
- No file size limit — reject files > 1 KiB
- No Unix permission check — warn/error if world-readable
- Non-`=` lines silently skipped — reject non-empty lines without `=`

### I1: Redundant `value.len() < 6` Check (INFORMATIONAL)
The length check is redundant with `is_encrypted()`. The edge case `"+encs+"` (exactly 6 chars,
empty ciphertext) produces a confusing `DecryptionFailed` instead of a clear error. Replace
with explicit empty-ciphertext check.

### I2: Salt Parsed But Unused (INFORMATIONAL)
Correct behavior — salt was used during key derivation, not needed for decryption.
Add explanatory comment for future maintainers.

### L4-partial: `Debug` Derive Leaks Crypto Details (LOW)
`EnvError` derives `Debug`, exposing `DecryptionFailed(String)` contents in panic output.
Consider manual `Debug` impl that redacts crypto-sensitive variants.

### L9: Key File Path Traversal (LOW)
`parse_key_file` passes the `path` argument directly to `File::open()` with no validation.
If `key_file` originates from untrusted input, arbitrary files could be read. The error at
line 32-33 echoes back unrecognized key names from the file, leaking left-hand side content
of `=`-delimited lines from arbitrary files. Practical impact is limited (format validation
rejects most files), but validate path against expected patterns or at minimum avoid echoing
file content in error messages.

## Scope

Changes to `src/secure_env.rs` and `src/env.rs`:

1. Collapse decrypt error paths 3 and 4 into single message
2. Normalize error messages to omit internal crypto details
3. Add duplicate key detection in `parse_key_file`
4. Add file size check before reading key file
5. Add Unix permission check (cfg(unix))
6. Reject non-empty lines without `=`
7. Fix `value.len() < 6` to explicit empty-ciphertext error
8. Add comment explaining salt is intentionally unused
9. Manual `Debug` impl for `EnvError` that redacts crypto details
10. Avoid echoing arbitrary file content in key file parse errors
