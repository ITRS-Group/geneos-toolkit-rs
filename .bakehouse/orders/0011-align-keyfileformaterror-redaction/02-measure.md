# Measure - Order #11 Align KeyFileFormatError redaction

## Context

Key-file format Display now emits `Key file format error` while Debug remains redacted; secure-env tests assert inner variants separately from safe public formatting.

Files in scope: `src/env.rs`, `src/secure_env.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
