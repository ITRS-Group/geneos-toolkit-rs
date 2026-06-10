# Served - Order #25 Fix ambiguous null escape in test

Replaced `"row\01"` with `"row\u{0}1"` in `src/dataview.rs`.

Proof: targeted test and full clippy command passed.
