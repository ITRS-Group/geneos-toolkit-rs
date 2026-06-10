# Review - Order #14

Reviewed scoped diff: `src/dataview.rs`, `src/env.rs`, `README.md`.

`DataviewError` and `EnvError` are annotated `#[non_exhaustive]`, and README API notes tell downstream users to keep wildcard match arms.

Decision: pass.
