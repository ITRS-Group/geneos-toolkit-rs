# Cook - Order #25 Fix ambiguous null escape in test

## Slice 1: Test literal cleanup

- Classification: test-only clippy cleanup.
- File changed: `src/dataview.rs`.

## Red

Command:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Result before fix: exit 101 with `clippy::octal_escapes` at `src/dataview.rs:1040` for `"row\01"`.

## Green

Changed the literal to `"row\u{0}1"`, preserving a null byte followed by `1` without ambiguous escape syntax.

Commands:

```sh
cargo test test_null_byte_in_row_name
cargo clippy --all-targets --all-features -- -D warnings
```

Results: both exit 0.
