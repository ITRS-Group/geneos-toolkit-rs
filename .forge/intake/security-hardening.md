# Security Hardening Initiative

Full security audit of geneos-toolkit-rs (v0.3.1) identified findings across three surfaces:
protocol injection, cryptographic module, and supply chain/CI. This initiative tracks
remediation of all confirmed findings.

## Audit Summary

| Severity | Count | Key Findings |
|----------|-------|-------------|
| Critical | 1 | `<!>` protocol injection via row names/headers |
| High | 3 | Null byte passthrough, CI tag-only, secure-env untested |
| Medium | 6 | Key material not zeroed, unicode control chars, cargo deny gaps |
| Low | 7 | Error leakage, padding oracle, keyfile parsing, empty names |
| Informational | 3 | Redundant length check, salt comment, crate category |

## Children (execution order)

1. **fix-protocol-injection** — CRITICAL: Escape `<!>` at line-start, null bytes, strip unicode control chars
2. **fix-ci-pipeline** — HIGH: Push/PR triggers, `--all-features`, deny config, permissions, action pinning
3. **harden-crypto-module** — LOW: Error normalization, padding oracle collapse, keyfile robustness
4. **add-zeroize-support** — MEDIUM: `zeroize` crate, `Zeroizing<String>` return type (breaking → 0.4.0)
5. **fix-crate-metadata** — INFO: Categories, MSRV policy

## Context

- Crate is used in production on financial services monitoring servers
- `secure-env` feature handles AES-256-CBC decryption of Geneos encrypted env vars
- Output consumed by Geneos Netprobe (C/C++ parser)
- Protocol format is fixed by Geneos — cannot change cipher suite or IV handling
