# Prep - Order #26 Replace while-let iterator loop in property test

## Slice 1: Property-test loop cleanup

- Classification: test-only clippy cleanup.
- File: `src/dataview.rs`.
- Proof: clippy red already showed `clippy::while_let_on_iterator`; after edit run the property test and full clippy command.
