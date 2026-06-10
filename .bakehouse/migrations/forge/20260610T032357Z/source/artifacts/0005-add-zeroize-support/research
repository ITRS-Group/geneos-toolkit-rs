# Research: add-zeroize-support

## Current State

### Key Material Lifecycle in `decrypt()`
1. `parse_key_file()` returns `(String, String, String)` — salt, key_hex, iv_hex
2. `Vec::from_hex(key_hex)` → `key_bytes: Vec<u8>` (32 bytes, the AES-256 key)
3. `Vec::from_hex(iv_hex)` → `iv_bytes: Vec<u8>` (16 bytes, the CBC IV)
4. `Vec::from_hex(hex)` → `encrypted_bytes: Vec<u8>` (ciphertext, becomes plaintext in-place)
5. `decrypt_padded_mut` decrypts in-place into `encrypted_bytes`, returns `&[u8]` slice
6. `String::from_utf8(decrypted_bytes.into())` allocates a new `String` from the plaintext

All of these are plain heap allocations. When dropped, memory is freed but not zeroed.

### Impact Surface
- `key_hex`, `iv_hex`: 64 and 32 char hex strings of the AES key/IV
- `key_bytes`, `iv_bytes`: raw crypto key material (32 and 16 bytes)
- `encrypted_bytes`: after in-place decryption, contains plaintext secret
- Return value: the decrypted secret as `String`

### `zeroize` Crate API
- `Zeroizing<T>`: wrapper that calls `T::zeroize()` on `Drop`
- Implements `Deref`/`DerefMut`, so `Zeroizing<String>` → `&String` → `&str` transparently
- `Zeroizing<Vec<u8>>` → `&Vec<u8>` → `&[u8]` transparently
- `DerefMut` enables `&mut Zeroizing<Vec<u8>>` to auto-coerce for `decrypt_padded_mut`

### Breaking Change Analysis
- `decrypt()` return changes from `Result<String, ...>` to `Result<Zeroizing<String>, ...>`
- `get_secure_var()` and `get_secure_var_or()` likewise
- Callers using `&str` (via `.as_str()`, `&*val`, or auto-deref) continue to work
- Callers storing in `let val: String = ...` must change to `Zeroizing<String>` or `.to_string()`
- Prelude should re-export `Zeroizing` so callers don't need a direct `zeroize` dependency

### Complexity
Low-medium. 1 file primarily affected (`secure_env.rs`), plus `Cargo.toml`, `lib.rs` re-export, README migration note. All tests need minor adaptation but mostly still pass via `Deref`.
