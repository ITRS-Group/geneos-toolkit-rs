# Baseline Test Coverage

Evaluate current test coverage and add regression-guarding tests before any security
hardening changes. This ensures every subsequent task in the initiative has a green
baseline to work against — any breakage is attributable to the change, not pre-existing gaps.

## Scope

### Evaluate current coverage
- Run tests with and without `--all-features` to identify gaps
- Map which public API surfaces have test coverage and which don't
- Identify edge cases in existing behavior that lack tests

### Areas likely needing baseline tests
- `escape_nasty_chars` — current behavior for all special characters (document what IS
  escaped before changing what SHOULD be escaped)
- `Dataview::Display` output format — exact output for representative dataviews
- `decrypt()` — success and failure paths with `secure-env` enabled
- `parse_key_file()` — valid and invalid key file formats
- `get_var` / `get_var_or` — encrypted detection, fallback behavior
- `get_secure_var` / `get_secure_var_or` — end-to-end decrypt paths
- Builder edge cases — duplicate rows, duplicate columns, headline overwrite behavior

### Constraints
- Tests must pass on current code (green state) — do NOT fix bugs in this task
- If a test reveals a bug, document it in the test with a comment but make the assertion
  match current (buggy) behavior so the baseline stays green
- Property tests already exist for escaping — evaluate whether they cover enough
