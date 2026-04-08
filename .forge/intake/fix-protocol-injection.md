# Fix Protocol Injection Vulnerabilities

## Findings Addressed

### C1: `<!>` Headline Injection (CRITICAL)
Row names and row headers starting with `<!>` render as fake headlines to the Netprobe parser.
`escape_nasty_chars` escapes `\`, `,`, `\n`, `\r` but not `<!>`. An attacker controlling input
to `add_value()` or `set_row_header()` can forge arbitrary headline metadata.

Attack scenario: Inject `<!>AlertSeverity,OK` to suppress critical alerts in financial monitoring.

### H1: Null Byte Passthrough (HIGH)
`\0` passes through unescaped. Geneos Netprobe is C/C++ — null bytes cause string truncation.
Value `"legitimate\0<!>INJECTED"` could truncate or cause undefined behavior.

### M3: Unicode Control Character Passthrough (MEDIUM)
RTL override (U+202E) makes `"KO"` display as `"OK"` — visual spoofing. Zero-width spaces
cause rule matching failures. BOM at output start can confuse encoding detection.

Decision: Strip non-ASCII control characters by default; add builder option to preserve.

### L5: Empty Row/Column Names (LOW)
Empty strings accepted for row/column names, creating ambiguous output (lines starting with `,`).

## Scope

All changes are in `src/dataview.rs`:

1. Enhance `escape_nasty_chars` to also escape `\0` as `\\0` and handle `<!>` at string start
2. Add Unicode control character stripping (categories Cc, Cf excluding ASCII whitespace)
3. Add builder option to disable Unicode stripping
4. Validate row names, column names, and row header are non-empty (or define behavior)
5. Update property tests to cover new escaping
6. Add targeted tests for each injection vector
