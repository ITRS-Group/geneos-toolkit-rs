# Served - Order #18 Standardize builder string parameter types

Builder and `Row` string inputs now consistently use `impl AsRef<str>`; numeric example values were converted to strings and README documents numeric conversion.

Proof: order-specific validation and full project validation passed; security audit completed with 0 findings.
