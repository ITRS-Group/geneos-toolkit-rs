# Cook - Order #9 Fix key-file TOCTOU race

## Slice 1: Descriptor-bound key-file validation

- Classification: security bug fix.
- Files changed: `src/secure_env.rs`.

## Red

Command:

```sh
cargo test --features secure-env test_parse_key_file_uses_open_file_descriptor_for_checks_and_read
```

Result: exit 101.

Meaningful failure: the new regression test referenced the not-yet-implemented private opener path and failed to compile with `cannot find function parse_key_file_with_opener`.

## Green

Implementation:

- Replaced `fs::metadata(path)` before open with an open-first flow.
- Added `open_key_file`, `parse_key_file_with_opener`, `parse_open_key_file`, and `validate_key_file_metadata` private helpers.
- `parse_key_file` now opens the file, validates `file.metadata()`, then parses from that same `File`.
- Kept existing redacted IO error text (`cannot open key file`) and key-file format messages.

Command:

```sh
cargo test --features secure-env test_parse_key_file_uses_open_file_descriptor_for_checks_and_read
```

Result: exit 0; 1 secure-env unit test passed.

## Refactor / formatting

- `cargo fmt --check` initially reported formatting for the new test.
- Ran `cargo fmt`.
- Later `cargo fmt --check` passed with exit 0.

## Validation run during Cook

```sh
cargo test --features secure-env
```

Result: exit 0; 63 passed, 1 ignored in unit tests; 2 integration tests passed; 6 doctests passed, 2 ignored.

```sh
cargo test
```

Result: exit 0; 43 passed in unit tests; 2 integration tests passed; 6 doctests passed, 2 ignored.

```sh
cargo clippy --lib --all-features -- -D warnings
```

Result: exit 0.

## Caveat

`cargo clippy --all-targets --all-features -- -D warnings` was run and failed on pre-existing, unrelated test-code findings in `src/dataview.rs`:

- `clippy::octal_escapes` at `src/dataview.rs:1040` (tracked by order #25).
- `clippy::while-let-on-iterator` at `src/dataview.rs:1350` (tracked by order #26).

Those findings were not changed here because this order's scope is the key-file TOCTOU bug.
