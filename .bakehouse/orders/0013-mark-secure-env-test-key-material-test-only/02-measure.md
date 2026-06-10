# Measure - Order #13 Mark secure-env test key material test-only

## Context

Secure-env test key material is explicitly labeled test-only and never for production use.

Files in scope: `src/secure_env.rs`.

## Constraints

No new runtime dependencies; preserve existing behavior unless the order explicitly requested an API/documentation change.
