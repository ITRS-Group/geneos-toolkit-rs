# Spec: add-zeroize-support

## Objective
Wrap all cryptographic key material and decrypted secrets in `Zeroizing<T>` so they are zeroed on drop. This is a breaking API change targeting v0.4.0.

## Acceptance Criteria

1. `key_bytes` and `iv_bytes` in `decrypt()` are `Zeroizing<Vec<u8>>`
2. `key_hex`, `iv_hex`, and `salt` from `parse_key_file()` are `Zeroizing<String>`
3. `encrypted_bytes` in `decrypt()` is `Zeroizing<Vec<u8>>`
4. `decrypt()` returns `Result<Zeroizing<String>, EnvError>`
5. `get_secure_var()` returns `Result<Zeroizing<String>, EnvError>`
6. `get_secure_var_or()` returns `Result<Zeroizing<String>, EnvError>`
7. `Zeroizing` is re-exported from `prelude`
8. All existing tests pass (adapted for `Zeroizing` return types)
9. Version bumped to 0.4.0
10. README updated with migration note and corrected version refs

## Out of Scope
- Custom `Zeroize` derive on `EnvError` or other types
- Zeroizing the `value` parameter in `decrypt()` (caller-owned `&str`)
- Changes to `env.rs` (no crypto material there)
