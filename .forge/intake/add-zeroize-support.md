# Add Zeroize Support

**Breaking change — targets 0.4.0 release.**

## Findings Addressed

### M1: Key Material Not Zeroed After Use (MEDIUM)
`key_bytes`, `iv_bytes`, `key_hex`, `iv_hex` are plain `Vec<u8>`/`String`. After `decrypt()`
returns, key material remains in heap until coincidentally overwritten. Recoverable from core
dumps, `/proc/pid/mem`, or swap on production financial services servers.

### M2: Decrypted Plaintext Returned as Plain `String` (MEDIUM)
`decrypt()` returns `Result<String, EnvError>`. The secret lives unprotected in heap, can be
logged with `{:?}`, swapped to disk. Changing to `Zeroizing<String>` provides automatic
zeroing on drop and a type-level signal that the value is sensitive.

### L4: `encrypted_bytes` Buffer Not Zeroed (LOW)
After in-place AES-CBC decryption, `encrypted_bytes` contains plaintext. If UTF-8 conversion
fails, the buffer is dropped without zeroing.

## Scope

Changes to `Cargo.toml`, `src/secure_env.rs`, `src/env.rs`:

1. Add `zeroize` as optional dependency behind `secure-env` feature
2. Wrap `key_bytes`, `iv_bytes` in `Zeroizing<Vec<u8>>`
3. Wrap `key_hex`, `iv_hex`, salt in `Zeroizing<String>`
4. Wrap `encrypted_bytes` in `Zeroizing<Vec<u8>>`
5. Change `decrypt()` return type to `Result<Zeroizing<String>, EnvError>`
6. Change `get_secure_var()` and `get_secure_var_or()` return types to match
7. Update all tests
8. Bump version to 0.4.0
9. Update README with migration note

Note: `Zeroizing<String>` implements `Deref<Target=String>`, so callers using `&str`
references continue to work. Callers storing the return value in a `String` variable
will need to update.
