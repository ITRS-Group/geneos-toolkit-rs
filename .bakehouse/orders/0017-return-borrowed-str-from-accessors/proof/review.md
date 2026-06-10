# Review - Order #17

Reviewed scoped diff: `src/dataview.rs`.

`Dataview::headline` and `Dataview::value` now return `Option<&str>` and tests compare against string slices.

Decision: pass.
