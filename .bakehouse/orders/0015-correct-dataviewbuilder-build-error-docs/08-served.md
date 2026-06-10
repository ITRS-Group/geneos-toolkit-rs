# Served - Order #15 Correct DataviewBuilder build error docs

`DataviewBuilder::build` rustdoc now says missing row headers return `DataviewError::MissingRowHeader` instead of panicking.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
