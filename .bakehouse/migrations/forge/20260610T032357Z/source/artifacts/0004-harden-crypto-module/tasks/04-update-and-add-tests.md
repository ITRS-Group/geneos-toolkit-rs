# Step 04: Update and Add Tests

## Objective
Update existing tests to match new error messages. Add new tests covering all hardening behaviors.

## Deliverables
- `src/secure_env.rs` (test module)
- `src/env.rs` (test module, if Debug test added here)

## Depends On
01-opaque-decrypt-errors, 02-harden-parse-key-file, 03-redact-debug-salt-comment

## Steps

1. Update `test_decrypt_missing_keyfile`: error no longer contains the path. Assert IoError with generic message.

2. Update `test_decrypt_invalid_hex`: still matches `DecryptionFailed(_)` — verify message is now "decryption failed".

3. Update `test_parse_key_file` invalid section: verify error does not echo the key name.

4. Add new tests:

   **Duplicate key rejection:**
   ```rust
   #[test]
   fn test_parse_key_file_duplicate_key() {
       // Write a key file with "key=" appearing twice
       // Assert KeyFileFormatError with "duplicate" in message
   }
   ```

   **Oversize file rejection:**
   ```rust
   #[test]
   fn test_parse_key_file_oversize() {
       // Write a file > 1024 bytes
       // Assert KeyFileFormatError with "too large" in message
   }
   ```

   **No-equals line rejection:**
   ```rust
   #[test]
   fn test_parse_key_file_no_equals_line() {
       // Write a file with a non-empty line lacking '='
       // Assert KeyFileFormatError with "invalid line" in message
   }
   ```

   **World-readable permission rejection (cfg(unix)):**
   ```rust
   #[cfg(unix)]
   #[test]
   fn test_parse_key_file_world_readable() {
       // Create key file with mode 0o644 (world-readable)
       // Assert KeyFileFormatError with "world-readable" in message
   }
   ```

   **Empty ciphertext:**
   ```rust
   #[test]
   fn test_decrypt_empty_ciphertext() {
       // decrypt("+encs+", key_file) should return DecryptionFailed("empty ciphertext")
   }
   ```

   **Debug redaction:**
   ```rust
   #[test]
   fn test_env_error_debug_redacts_crypto() {
       // Create DecryptionFailed("secret details")
       // Assert format!("{:?}", err) contains "[REDACTED]" and not "secret details"
   }
   ```

## Acceptance Criteria
- AC11: All existing tests pass with updated assertions
- AC12: New tests cover duplicate key, oversize file, no-`=` line, world-readable, empty ciphertext, Debug redaction
