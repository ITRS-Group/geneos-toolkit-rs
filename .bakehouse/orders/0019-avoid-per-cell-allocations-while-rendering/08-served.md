# Served - Order #19 Avoid per-cell allocations while rendering

Dataview cell storage is now nested by row then column, allowing render lookups by borrowed `&str` without allocating `(String, String)` keys per cell.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
