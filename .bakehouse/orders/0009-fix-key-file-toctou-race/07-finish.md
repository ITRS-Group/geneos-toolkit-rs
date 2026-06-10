# Finish - Order #9 Fix key-file TOCTOU race

## Outcome

`parse_key_file` now binds key-file metadata checks and parsing to one opened file descriptor. This removes the path/symlink swap window between metadata validation and read.

## Changed files

- `src/secure_env.rs`
  - open-first key-file flow
  - descriptor-bound metadata validation
  - deterministic Unix regression test for path replacement after open
- `.bakehouse/orders/0009-fix-key-file-toctou-race/*`
  - Measure/Recipe/Prep/Cook/Taste/Finish/Served proof and security audit report

## Evidence

- Red test: targeted secure-env regression failed before helper implementation (exit 101).
- Green test: targeted regression passed (exit 0).
- `cargo test --features secure-env` passed (exit 0).
- `cargo test` passed (exit 0).
- `cargo fmt --check` passed (exit 0).
- `cargo clippy --lib --all-features -- -D warnings` passed (exit 0).
- Security audit for `src/secure_env.rs` passed with 0 findings.

## Waiver / caveat

`cargo clippy --all-targets --all-features -- -D warnings` did not pass because of unrelated pre-existing findings in `src/dataview.rs`:

- Order #25: ambiguous null escape in a test string.
- Order #26: `while let` iterator loop in a property test.

These are explicitly outside order #9 scope and are already represented by separate orders.

## Follow-up

Bake orders #25 and #26 to clear the all-targets clippy gate.
