# Measure - Order #12 Correct rust-toolchain pin comments

## Context

`.github/workflows/deploy.yaml` no longer labels pinned dtolnay/rust-toolchain commits as `master`; comments now identify a pinned commit.

Files in scope: `.github/workflows/deploy.yaml`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
