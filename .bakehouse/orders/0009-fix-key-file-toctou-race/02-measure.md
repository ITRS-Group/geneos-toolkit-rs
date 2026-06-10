# Measure - Order #9 Fix key-file TOCTOU race

## Context checked

- Target: `src/secure_env.rs`, gated by the optional `secure-env` feature.
- Current `parse_key_file(path)` calls `fs::metadata(path)` before `File::open(path)`, then parses the later-opened file via `BufReader`.
- Existing secure-env tests cover valid parsing, invalid lines, duplicate keys, oversize files, world-readable files on Unix, missing files, decryption behavior, and Display/Debug redaction.

## Security finding

- Risk: a path or symlink can be swapped between `fs::metadata(path)` and `File::open(path)`, so size and Unix world-readability checks can apply to one file while parsing reads another.
- Desired binding: open the key file first, then use `file.metadata()` on that same descriptor before constructing the reader.
- Sensitive-data boundary: key-file paths and contents must remain absent from user-visible errors; existing redaction tests must continue to pass.

## Constraints

- Preserve MSRV from `Cargo.toml` (`rust-version = "1.85"`).
- Do not add runtime dependencies; default build must remain dependency-free outside optional features.
- Keep the change narrow to key-file opening/metadata validation and tests.

## Proof plan

- Add a focused secure-env regression test that opens a key file, replaces the path before validation/parsing, and proves the opened descriptor is used for both checks and read.
- Run targeted secure-env tests, full `cargo test --features secure-env`, and `cargo clippy --all-targets --all-features -- -D warnings`.
