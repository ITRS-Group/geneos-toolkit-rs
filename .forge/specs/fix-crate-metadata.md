# Fix Crate Metadata

## Objective

Correct Cargo.toml metadata so the crate is accurately categorized on crates.io
and declares its minimum supported Rust version (MSRV).

## Acceptance Criteria

- [ ] `categories` changed from `["command-line-utilities"]` to `["api-bindings"]`
- [ ] `rust-version = "1.85"` field added (edition 2024 minimum)
- [ ] `cargo check` passes with no warnings
- [ ] `cargo test --all-features` passes

## Technical Constraints

- Single file change: `Cargo.toml`
- No functional code changes
- MSRV value derived from edition 2024 requirement (Rust 1.85.0)

## Out of Scope

- Adding MSRV check to CI (separate follow-up if desired)
- Changing keywords or other optional metadata fields
