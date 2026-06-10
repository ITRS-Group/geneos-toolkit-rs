# Measure - Order #25 Fix ambiguous null escape in test

Clippy reported `clippy::octal_escapes` for `src/dataview.rs` test literal `"row\01"`. In Rust, `\0` is a null escape and the following `1` makes the literal visually ambiguous.

The test intends to cover a row name containing a null byte adjacent to `1`; an explicit Unicode escape preserves that intent.
