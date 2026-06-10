# Taste - Order #25 Fix ambiguous null escape in test

## Acceptance criteria

- AC-001: Passed. The ambiguous `"row\01"` literal was replaced with `"row\u{0}1"`.
- AC-002: Passed. The test still creates a row name containing a null byte followed by `1`.
- AC-003: Passed. `cargo clippy --all-targets --all-features -- -D warnings` exited 0.

Decision: pass.
