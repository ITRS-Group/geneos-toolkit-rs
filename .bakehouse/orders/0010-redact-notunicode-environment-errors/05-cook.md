# Cook - Order #10 Redact NotUnicode environment errors

## Slice 1: EnvError redaction

- Classification: security bug fix.
- Files changed: `src/env.rs`.

### Red

Command:

```sh
cargo test notunicode -- --nocapture
```

Result: exit 101. The new tests failed because Display delegated to `std::env::VarError` and exposed `"secret-\\xFF-token"`.

### Green

Implementation:

- Added a dedicated Debug arm for `EnvError::VarError(NotUnicode(_))`: `VarError(NotUnicode([REDACTED]))`.
- Added a fixed Display message: `Environment variable error: environment variable value is not valid Unicode`.
- Changed `Error::source` to return `None` for NotUnicode.
- Added Unix regression coverage for direct `EnvError`, `get_var`, and `get_var_or` paths.

Command:

```sh
cargo test notunicode -- --nocapture
```

Result: exit 0; 2 env tests passed.

## Slice 2: Secure-env propagation coverage

- Classification: feature-gated regression test.
- Files changed: `src/secure_env.rs`.

Command:

```sh
cargo test --features secure-env not_unicode -- --nocapture
```

Result: exit 0; secure-env invalid-Unicode path test passed.

## Cross-order clippy blockers

To satisfy AC-005 without a waiver, the same `/bake all` run also fixed orders #25 and #26 in `src/dataview.rs`:

- Replaced ambiguous `"row\\01"` test literal with `"row\\u{0}1"`.
- Replaced a `while let Some(c) = chars.next()` property-test loop with `for c in line.chars()`.

## Validation

- `cargo fmt --check` — exit 0.
- `cargo test` — exit 0; 45 unit tests, 2 integration tests, 6 doctests passed; 2 doctests ignored.
- `cargo test --features secure-env` — exit 0; 66 unit tests, 2 integration tests, 6 doctests passed; 2 doctests ignored.
- `cargo clippy --all-targets --all-features -- -D warnings` — exit 0.
