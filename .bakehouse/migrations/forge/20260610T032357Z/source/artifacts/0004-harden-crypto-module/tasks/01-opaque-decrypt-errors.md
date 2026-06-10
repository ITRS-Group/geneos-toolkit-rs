# Step 01: Opaque Decrypt Errors

## Objective
Collapse all `DecryptionFailed` error paths in `decrypt()` to a single opaque message. Add explicit empty-ciphertext handling. Remove redundant length check.

## Deliverables
- `src/secure_env.rs`

## Depends On
None

## Steps

1. In `decrypt()`, remove `value.len() < 6` from the guard (line 54). Keep only `!is_encrypted(value)`.

2. After extracting `hex = &value[6..]`, add an empty-ciphertext check:
   ```rust
   if hex.is_empty() {
       return Err(EnvError::DecryptionFailed("empty ciphertext".to_string()));
   }
   ```

3. Collapse all remaining `DecryptionFailed` error paths (lines 60, 65, 67, 72, 74, 77) to use the same opaque message `"decryption failed"`:
   ```rust
   .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?;
   ```

## Acceptance Criteria
- AC1: All DecryptionFailed paths produce "decryption failed" (no hex/padding/UTF-8 details)
- AC7: "+encs+" returns DecryptionFailed("empty ciphertext")
- AC8: Redundant length check removed
