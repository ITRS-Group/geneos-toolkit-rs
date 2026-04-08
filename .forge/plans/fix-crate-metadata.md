# Plan: fix-crate-metadata

## Metadata

- **Slug:** fix-crate-metadata
- **Spec:** specs/fix-crate-metadata.md
- **Branch:** master (standalone metadata fix, no feature branch needed)

## Scope Manifest

- **Files modified:** `Cargo.toml`
- **Files created:** none
- **Interactive regression checkpoint:** not required

## Phase 2: Implementation

### Step 1: Update Cargo.toml metadata

1. Change `categories = ["command-line-utilities"]` to `categories = ["api-bindings"]`
2. Add `rust-version = "1.85"` after the `edition` field
3. Run `cargo check` and `cargo test --all-features` to verify
