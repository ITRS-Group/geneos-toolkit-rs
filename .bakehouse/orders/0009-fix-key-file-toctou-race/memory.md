# Order Memory

## Current Understanding

Order #9 fixed the secure-env key-file TOCTOU finding by making metadata validation descriptor-bound.

## Decisions

- Used an opener helper to make the race regression deterministic without adding dependencies or public API.
- Preserved existing redacted IO and key-file error messages.
- Waived the all-targets clippy gate for unrelated pre-existing dataview test findings tracked by orders #25 and #26.

## Resume Notes

Order is served. If all-targets clippy proof is needed without waiver, bake orders #25 and #26 first.
