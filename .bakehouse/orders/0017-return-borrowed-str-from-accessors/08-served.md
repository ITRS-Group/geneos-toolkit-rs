# Served - Order #17 Return borrowed str from accessors

`Dataview::headline` and `Dataview::value` now return `Option<&str>` and tests compare against string slices.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
