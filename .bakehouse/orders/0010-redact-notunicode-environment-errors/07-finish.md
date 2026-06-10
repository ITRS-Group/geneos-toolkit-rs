# Finish - Order #10 Redact NotUnicode environment errors

## Outcome

Invalid-Unicode environment variable errors are now safe to format:

- Display uses fixed, non-secret text.
- Debug uses a fixed redacted representation.
- `Error::source` does not expose the inner `std::env::VarError` for NotUnicode.

## Evidence

- `cargo test notunicode -- --nocapture` red/green recorded in Cook.
- `cargo test --features secure-env not_unicode -- --nocapture` passed.
- `cargo test` passed.
- `cargo test --features secure-env` passed.
- `cargo fmt --check` passed.
- `cargo clippy --all-targets --all-features -- -D warnings` passed.
- Security audit passed with 0 findings.

## Follow-up

No order-specific follow-up. Orders #25 and #26 were also completed in this `/bake all` run to clear pre-existing clippy blockers.
