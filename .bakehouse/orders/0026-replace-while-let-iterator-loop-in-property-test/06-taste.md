# Taste - Order #26 Replace while-let iterator loop in property test

## Acceptance criteria

- AC-001: Passed. The `while let Some(c) = chars.next()` loop was replaced with a `for c in line.chars()` loop.
- AC-002: Passed. The comma-counting escaped-state logic is unchanged and the property test passes.
- AC-003: Passed. `cargo clippy --all-targets --all-features -- -D warnings` exited 0.

Decision: pass.
