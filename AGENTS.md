# AGENTS.md

Guidance for AI coding agents working in this repository. Human-facing
documentation lives in [`README.md`](./README.md).

## What this crate is

`geneos-toolkit` is a Rust library for building Geneos Toolkit-compatible
monitoring scripts. The core abstraction is the `Dataview` builder (see
`src/dataview.rs`); environment variable helpers live in `src/env.rs`; an
optional `secure-env` feature gates encrypted-env support (`src/secure_env.rs`).

The crate aims to stay **zero third-party runtime dependencies by default** —
encryption crates only compile in when `secure-env` is enabled. Preserve this
when adding code: do not pull in new runtime dependencies without explicit
discussion. `dev-dependencies` are fine for tests and examples.

## Commands

```sh
cargo build                            # build the library
cargo test                             # run unit + integration tests
cargo test --features secure-env       # include the secure-env code paths
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --example <name>             # run any example in examples/
```

Integration tests in `tests/examples_test.rs` shell out to `cargo run --example`
and assert on stdout — add new entries here when you add an example whose
output is deterministic.

## Examples (`examples/`)

The examples are both documentation and copy-paste starting points. Two
flavours:

**Library API demos** — short, deterministic, asserted by `tests/examples_test.rs`:

- `basic_dataview.rs` — minimal dataview
- `dataview_with_commas_in_cells.rs` — comma escaping
- `dataview_with_multiple_headlines.rs` — headline ordering
- `row_builder.rs` — fluent `Row` builder
- `iterative_rows.rs` — building rows in a loop
- `files_iter.rs` — directory iteration
- `readme_iterative_rows.rs` — keeps the README iterative-rows snippet honest

**Full application monitors** — realistic Toolkit scripts using only the stdlib
plus external CLI tools:

- `redis_info_monitor.rs` — parses `redis-cli INFO` into a dataview. Demonstrates
  command exec, multi-section text parsing, derived metrics (hit ratio),
  conditional headlines (replica-only), numeric `sort_rows_by`, and a placeholder
  row for empty data. Reads `REDIS_HOST` / `REDIS_PORT` / `REDIS_PASSWORD` /
  `REDIS_CLI_BIN`. Also demonstrates the `secure-env` feature end-to-end: build
  with `--features secure-env` and set `REDIS_KEY_FILE` to decrypt a Geneos
  `+encs+...` `REDIS_PASSWORD` into a `Zeroizing<String>`. Excluded from
  `test_all_examples_compile`'s run pass (no `redis-cli` in CI); still compiled.

When adding a new full-app example, follow the same conventions: read config
from the environment via `get_var_or` (or `get_secure_var_or` when handling
secrets under `secure-env`), shell out via `std::process::Command`, pass secrets
through `cmd.env(...)` rather than argv, validate inputs that flow into command
args (reject `-` prefixes, parse numeric values), and exit nonzero with an
`ERROR: ...` line on stderr on failure. If the example needs external
infrastructure to run, add its name to `SKIP_RUN` in `tests/examples_test.rs`.

## Conventions

- **Version tags**: bare (`0.4.0`), not `v`-prefixed.
- **Errors**: explicit `Result` returns; no panics in library code. Examples
  may `unwrap` where the failure is clearly the user's fault (missing config).
- **Security**: see the audit notes referenced from the agent's own memory if
  available; in particular, all user input that lands in a dataview must go
  through the builder's sanitisation (already handled — do not bypass it).
- **MSRV**: keep compatible with the `rust-version` field in `Cargo.toml`.
