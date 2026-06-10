# Plan: add-zeroize-support

## Steps

### 01: Add zeroize dependency
- `Cargo.toml`: add `zeroize = { version = "1", optional = true }` to `[dependencies]`
- Add `"zeroize"` to the `secure-env` feature list

### 02: Wrap key material in secure_env.rs
- Add `use zeroize::Zeroizing;`
- `parse_key_file()`: change return to `Result<(Zeroizing<String>, Zeroizing<String>, Zeroizing<String>), EnvError>`
  - Wrap each `Some(value.to_string())` in `Zeroizing::new(...)`
  - Wrap final `Ok(...)` values
- `decrypt()`:
  - `encrypted_bytes` → `Zeroizing::new(Vec::from_hex(...))`
  - `key_bytes`, `iv_bytes` → `Zeroizing::new(Vec::from_hex(...))`
  - Return type → `Result<Zeroizing<String>, EnvError>`
  - Non-encrypted passthrough: `Ok(Zeroizing::new(value.to_string()))`
  - Decrypted result: `Ok(Zeroizing::new(String::from_utf8(...)?))`
- `get_secure_var()` → `Result<Zeroizing<String>, EnvError>`
  - Non-encrypted branch: `Ok(Zeroizing::new(value))`
- `get_secure_var_or()` → `Result<Zeroizing<String>, EnvError>`
  - Non-encrypted branch: `Ok(Zeroizing::new(val))`
  - Default branch: `Ok(Zeroizing::new(default.to_string()))`

### 03: Update prelude and tests
- `lib.rs`: add `pub use zeroize::Zeroizing;` to prelude (behind `secure-env` feature)
- Tests: adapt assertions — most work via `Deref`, but `assert_eq!` on `Zeroizing<String>` vs `&str` needs `.as_str()` or `&*result`

### 04: Version bump and README
- `Cargo.toml`: version 0.3.1 → 0.4.0
- `README.md`: update version references, add migration note for `Zeroizing<String>` return type change

## Build Sequence
01 → 02 → 03 → 04 (sequential, each depends on prior)
