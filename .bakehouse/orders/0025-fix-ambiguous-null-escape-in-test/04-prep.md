# Prep - Order #25 Fix ambiguous null escape in test

## Slice 1: Test literal cleanup

- Classification: test-only clippy cleanup.
- File: `src/dataview.rs`.
- Proof: clippy red already showed `clippy::octal_escapes`; after edit run the targeted null-byte row-name test and full clippy command.
