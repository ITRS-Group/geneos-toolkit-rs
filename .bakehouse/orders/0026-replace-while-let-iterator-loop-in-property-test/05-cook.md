# Cook - Order #26 Replace while-let iterator loop in property test

## Slice 1: Property-test loop cleanup

- Classification: test-only clippy cleanup.
- File changed: `src/dataview.rs`.

## Red

Command:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Result before fix: exit 101 with `clippy::while_let_on_iterator` at `src/dataview.rs:1350`.

## Green

Changed the raw comma counting loop to `for c in line.chars()` and left the `escaped` state-machine logic unchanged.

Commands:

```sh
cargo test test_dataview_column_count_consistency
cargo clippy --all-targets --all-features -- -D warnings
```

Results: both exit 0.
