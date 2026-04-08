# Fix CI/CD Pipeline

## Findings Addressed

### H2: CI Only Triggers on Tags (HIGH)
No CI on pushes to master or PRs. Code breakage discovered at release time only.
Modify existing workflow to also trigger on push to master and pull requests.

### H3: `secure-env` Feature Never Tested in CI (HIGH)
`cargo test` and `cargo clippy` run without `--all-features`. The entire AES-256-CBC
decryption module is never compiled or tested by CI. Add `--all-features` variants.

### M5: `cargo deny` Partial Check + Minimal Config (MEDIUM)
CI runs `cargo deny check advisories` — not the full check. The license allow-list is
never verified. Enhance deny.toml with `[bans]` and `[sources]` sections. Change CI
command to `cargo deny check`.

### M6: GitHub Actions Permissions + Mutable Action Tags (MEDIUM)
No explicit `permissions:` block (defaults to read-write). All third-party actions pinned
to mutable tags. Add minimal permissions, pin actions to full commit SHAs.

### L6: Publish Token on Command Line (LOW)
`cargo publish --token ${CRATES_IO_TOKEN}` exposes token in `/proc/<pid>/cmdline`.
Use `CARGO_REGISTRY_TOKEN` env var instead.

## Scope

Changes to `.github/workflows/deploy.yaml` and `deny.toml`:

1. Add `push: branches: [master]` and `pull_request: branches: [master]` triggers
2. Add `--all-features` test and clippy steps alongside default-features steps
3. Change `cargo deny check advisories` to `cargo deny check`
4. Enhance deny.toml with `[bans]` and `[sources]` sections
5. Add `permissions: contents: read` to workflow
6. Pin all third-party actions to full commit SHAs
7. Switch publish step to `CARGO_REGISTRY_TOKEN` env var
