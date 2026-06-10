# Recipe - Order #25 Fix ambiguous null escape in test

Replace the ambiguous test literal with an unambiguous null-byte escape:

- Before: `"row\01"`
- After: `"row\u{0}1"`

This keeps the same row-name shape (`row`, null byte, `1`) while satisfying clippy.
