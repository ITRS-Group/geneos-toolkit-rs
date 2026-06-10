# Fix Crate Metadata

## Findings Addressed

### I3: Crate Category Incorrect (INFORMATIONAL)
`categories = ["command-line-utilities"]` but the crate is a library. Should be
`["api-bindings"]` or `["development-tools"]` or similar.

### L7: No MSRV Policy (LOW)
`edition = "2024"` implies Rust 1.85+ but no `rust-version` field in Cargo.toml.
Production deployments in regulated environments need known compiler version requirements.

## Scope

Changes to `Cargo.toml`:

1. Change `categories` to appropriate library category
2. Add `rust-version = "1.85"` (or verify actual minimum)
3. Optionally add MSRV check to CI (if CI task is complete by then)
