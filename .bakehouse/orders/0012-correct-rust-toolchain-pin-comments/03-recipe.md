# Recipe - Order #12 Correct rust-toolchain pin comments

## Approach

Implement the requested workflow comment correction in the scoped files, keep the diff narrow, and validate with the order-specific command(s) plus the full project validation suite.

## Expected proof

- `! rg "dtolnay/rust-toolchain@.*# master" .github/workflows/deploy.yaml` — exit 0.
