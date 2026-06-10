# Served - Order #14 Make public error enums non-exhaustive

`DataviewError` and `EnvError` are annotated `#[non_exhaustive]`, and README API notes tell downstream users to keep wildcard match arms.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
