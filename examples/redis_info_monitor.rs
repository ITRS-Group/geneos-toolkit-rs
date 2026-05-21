//! Redis INFO monitor — a realistic Geneos Toolkit script.
//!
//! Shells out to `redis-cli INFO`, parses the multi-section response, and emits
//! a dataview with headlines for the most-watched metrics plus one row per
//! logical database.
//!
//! # Configuration
//!
//! Geneos passes sampler parameters as environment variables. This example
//! reads:
//!
//!   REDIS_CLI_BIN    path to redis-cli         (required; must contain a
//!                                                path separator — see below)
//!   REDIS_HOST       server hostname           (default: "127.0.0.1")
//!   REDIS_PORT       server port               (default: "6379")
//!   REDIS_PASSWORD   AUTH password, optional   (see "Secrets" below)
//!   REDIS_KEY_FILE   AES key file path         (required with `secure-env`
//!                                                when REDIS_PASSWORD is set)
//!
//! # Secrets
//!
//! How `REDIS_PASSWORD` is handled depends on the crate features:
//!
//! - **Default build** (`cargo run --example redis_info_monitor`)
//!   `REDIS_PASSWORD` must be a plain string. If the Gateway encrypts it
//!   (`+encs+...`), the script aborts with `MissingSecureEnvSupport` rather
//!   than silently authenticating as no-one. The plaintext lives in a plain
//!   `String` on the heap and is not zeroed when dropped — enable the
//!   `secure-env` feature whenever that matters.
//!
//! - **`secure-env` enabled**
//!   (`cargo run --example redis_info_monitor --features secure-env`)
//!   `REDIS_PASSWORD` may be a Geneos `+encs+...` ciphertext; it is decrypted
//!   using the AES key file at `REDIS_KEY_FILE`. The decrypted secret lives
//!   in a `Zeroizing<String>` and is wiped on drop. This is the recommended
//!   deployment for any production Geneos integration.
//!
//! Either way, the password is passed to `redis-cli` via the `REDISCLI_AUTH`
//! environment variable, not argv, so it does not appear in `ps` or
//! `/proc/<pid>/cmdline`. Note that it is still readable via
//! `/proc/<pid>/environ` by processes running as the same uid, and that
//! `Command::env` will hold a non-zeroed copy in the parent's heap until the
//! `Command` is dropped — `Zeroizing` is best-effort, not absolute.
//!
//! # Operational notes
//!
//! - `REDIS_CLI_BIN` is intentionally required and must contain a path
//!   separator (e.g. `/usr/bin/redis-cli`, not `redis-cli`). A bare filename
//!   would trigger `$PATH` resolution, which is a foot-gun on hosts where an
//!   attacker can drop a binary into a writable PATH entry. Forcing an
//!   explicit path moves the decision from "wherever PATH happens to point"
//!   to "wherever the operator deliberately chose."
//! - Exit codes: 0 on success (dataview written to stdout), 1 on any failure
//!   (with an `ERROR: ...` line on stderr).
//!
//! # Output
//!
//! Sample dataview against a Redis master with two populated databases:
//!
//! ```text
//! keyspace,keys,expires,avgTtlMs
//! <!>target,cache.prod:6379
//! <!>role,master
//! <!>redisVersion,7.2.4
//! <!>uptimeSeconds,84321
//! <!>connectedClients,42
//! <!>usedMemoryHuman,128.45M
//! <!>usedMemoryPeakHuman,201.10M
//! <!>totalCommandsProcessed,9876543
//! <!>instantaneousOpsPerSec,1287
//! <!>hitRatioPct,98.61
//! <!>evictedKeys,1247
//! <!>rejectedConnections,3
//! <!>keyspaceDbs,2
//! db0,152340,12,4500
//! db1,88,0,0
//! ```
//!
//! Column and headline names follow the camelCase convention used by Geneos's
//! built-in plugins, so this dataview reads consistently next to Hardware,
//! CPU, and Disk dataviews under the same Managed Entity.
//!
//! When the instance is a replica, an additional `<!>masterLinkStatus,...`
//! headline is emitted. When no databases hold data, a single placeholder
//! row `(none),0` is emitted alongside `<!>keyspaceDbs,0`.

use geneos_toolkit::dataview::DataviewError;
use geneos_toolkit::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::process::Command;

/// Server-response timeout passed to `redis-cli -t`. A hung server should not
/// pin the sampler indefinitely.
const TIMEOUT_SECS: &str = "5";

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli =
        get_var("REDIS_CLI_BIN").map_err(|_| "REDIS_CLI_BIN must be set to a path to redis-cli")?;
    // Reject bare filenames on any platform — `/` on Unix, `\` on Windows.
    // Path::components() counts > 1 iff the value names a directory component
    // (`./foo`, `bin/foo`, `/usr/bin/foo`, `C:\bin\foo`, ...).
    if Path::new(&cli).components().count() < 2 {
        return Err(format!(
            "REDIS_CLI_BIN must contain a path separator to avoid PATH lookup: {cli:?}"
        )
        .into());
    }
    let host = get_var_or("REDIS_HOST", "127.0.0.1")?;
    reject_flag_lookalike("REDIS_HOST", &host)?;
    let port = get_var_or("REDIS_PORT", "6379")?;
    port.parse::<u16>()
        .map_err(|_| format!("REDIS_PORT is not a valid u16: {port:?}"))?;
    let password = read_password()?;

    let info = run_redis_info(&cli, &host, &port, &password)?;
    let fields = parse_info(&info);
    let dataview = build_dataview(&host, &port, &fields)?;
    println!("{dataview}");
    Ok(())
}

/// Reads `REDIS_PASSWORD`. With the `secure-env` feature enabled, decrypts
/// `+encs+...` ciphertexts using the key file at `REDIS_KEY_FILE` and returns
/// a `Zeroizing<String>` so the plaintext is wiped on drop. Without the
/// feature, plain values pass through and encrypted values are rejected (via
/// `MissingSecureEnvSupport` from `get_var_or`).
#[cfg(not(feature = "secure-env"))]
fn read_password() -> Result<String, Box<dyn Error>> {
    // NB: plaintext lives in a plain `String` on the heap and is not zeroed
    // when dropped. Enable the `secure-env` feature to wrap the value in
    // `Zeroizing<String>` (which still leaves a non-zeroed copy in the
    // child's env table — see the module doc for the full caveat).
    Ok(get_var_or("REDIS_PASSWORD", "")?)
}

#[cfg(feature = "secure-env")]
fn read_password() -> Result<Zeroizing<String>, Box<dyn Error>> {
    let key_file = get_var_or("REDIS_KEY_FILE", "")?;
    if key_file.is_empty() {
        return Err("REDIS_KEY_FILE must be set when the `secure-env` feature is enabled".into());
    }
    Ok(get_secure_var_or("REDIS_PASSWORD", &key_file, "")?)
}

/// Rejects values that would be interpreted as a flag by `redis-cli`.
/// A cheap argv-injection guard: an attacker who controls `REDIS_HOST` cannot
/// turn it into a `-x` short option that changes redis-cli behaviour.
fn reject_flag_lookalike(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    if value.starts_with('-') {
        return Err(format!("{name} must not start with '-': {value:?}").into());
    }
    Ok(())
}

fn run_redis_info(
    cli: &str,
    host: &str,
    port: &str,
    password: &str,
) -> Result<String, Box<dyn Error>> {
    let mut cmd = Command::new(cli);
    cmd.args(["-h", host, "-p", port, "-t", TIMEOUT_SECS, "INFO"]);
    if !password.is_empty() {
        cmd.env("REDISCLI_AUTH", password);
    }

    // `Command::output` reads the child's stdout fully into memory without
    // a cap. A hostile Redis returning gigabytes of INFO could OOM this
    // process. The `-t 5` argument bounds per-read stalls but not total
    // volume; production deployments handling untrusted instances should
    // replace this with a bounded streamed read.
    let output = cmd
        .output()
        .map_err(|e| format!("failed to invoke redis-cli: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("redis-cli exited with {}: {}", output.status, stderr.trim()).into());
    }

    // The error type is intentionally collapsed: `FromUtf8Error`'s Display
    // includes a snippet of the offending bytes, which here are attacker-
    // controlled. Don't propagate that into stderr.
    String::from_utf8(output.stdout).map_err(|_| "non-UTF-8 INFO output".into())
}

/// Returns a flat map of every `key:value` line in the INFO response.
/// Section headers (`# Server`, `# Clients`, …) and blank lines are skipped.
/// `db0:keys=…,expires=…,avg_ttl=…` lines from the Keyspace section land in
/// the same flat map and are split apart by `collect_keyspace_rows`.
fn parse_info(info: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    for line in info.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            fields.insert(key.to_string(), value.to_string());
        }
    }
    fields
}

fn build_dataview(
    host: &str,
    port: &str,
    fields: &HashMap<String, String>,
) -> Result<Dataview, DataviewError> {
    let keyspace = collect_keyspace_rows(fields);

    // Headline and column names use camelCase to match the convention of
    // Geneos's built-in plugins (Hardware, CPU, Disk, Processes — see
    // `percentUtilisation`, `clockSpeed`, `kBytesUsed`). Sticking to the
    // house style means this dataview slots in cleanly next to built-in
    // dataviews under the same Managed Entity, and rules written against
    // built-in column names port over directly. The Redis INFO keys we read
    // *from* are left in their native snake_case form below.
    let mut builder = Dataview::builder()
        .set_row_header("keyspace")
        .add_headline("target", format!("{host}:{port}"))
        .add_headline("role", field_or_empty(fields, "role"))
        .add_headline("redisVersion", field_or_empty(fields, "redis_version"))
        .add_headline("uptimeSeconds", field_or_empty(fields, "uptime_in_seconds"))
        .add_headline(
            "connectedClients",
            field_or_empty(fields, "connected_clients"),
        )
        .add_headline(
            "usedMemoryHuman",
            field_or_empty(fields, "used_memory_human"),
        )
        .add_headline(
            "usedMemoryPeakHuman",
            field_or_empty(fields, "used_memory_peak_human"),
        )
        .add_headline(
            "totalCommandsProcessed",
            field_or_empty(fields, "total_commands_processed"),
        )
        .add_headline(
            "instantaneousOpsPerSec",
            field_or_empty(fields, "instantaneous_ops_per_sec"),
        )
        .add_headline("hitRatioPct", hit_ratio_pct(fields))
        .add_headline("evictedKeys", field_or_empty(fields, "evicted_keys"))
        .add_headline(
            "rejectedConnections",
            field_or_empty(fields, "rejected_connections"),
        )
        .add_headline("keyspaceDbs", keyspace.len().to_string());

    // Emit `masterLinkStatus` only when INFO actually reports it. Keying off
    // the field's presence is more robust than checking `role`, whose value
    // is `slave` in current Redis and may change in future versions.
    if let Some(status) = fields.get("master_link_status") {
        builder = builder.add_headline("masterLinkStatus", status);
    }

    if keyspace.is_empty() {
        // Dataview::build requires at least one row; the `keyspaceDbs=0`
        // headline above signals that this placeholder represents no data.
        builder = builder.add_value("(none)", "keys", "0");
    } else {
        for (db, raw) in keyspace {
            let parts: HashMap<&str, &str> =
                raw.split(',').filter_map(|kv| kv.split_once('=')).collect();
            builder = builder.add_row(
                Row::new(db)
                    .add_cell("keys", parts.get("keys").copied().unwrap_or("0"))
                    .add_cell("expires", parts.get("expires").copied().unwrap_or("0"))
                    .add_cell("avgTtlMs", parts.get("avg_ttl").copied().unwrap_or("0")),
            );
        }
    }

    builder
        .sort_rows_by(|name| {
            // Numeric sort so db10 follows db9, not db1.
            name.strip_prefix("db")
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or(u32::MAX)
        })
        .build()
}

/// Returns each `db<N>:...` line from the INFO response, paired as
/// `(db<N>, raw_value)`. Order is not preserved; the builder sorts later.
fn collect_keyspace_rows(fields: &HashMap<String, String>) -> Vec<(&str, &str)> {
    fields
        .iter()
        .filter(|(k, _)| {
            k.strip_prefix("db")
                .is_some_and(|rest| !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()))
        })
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect()
}

fn hit_ratio_pct(fields: &HashMap<String, String>) -> String {
    let hits = parse_u64(fields, "keyspace_hits");
    let misses = parse_u64(fields, "keyspace_misses");
    let total = hits.saturating_add(misses);
    if total == 0 {
        String::new()
    } else {
        format!("{:.2}", (hits as f64 / total as f64) * 100.0)
    }
}

fn parse_u64(fields: &HashMap<String, String>, key: &str) -> u64 {
    fields.get(key).and_then(|v| v.parse().ok()).unwrap_or(0)
}

/// Looks up a headline source field, returning an empty string when absent.
/// Empty headlines render as `<!>name,` — the Toolkit convention for "metric
/// not collected" — rather than polluting Geneos rules with an `"n/a"` literal.
fn field_or_empty(fields: &HashMap<String, String>, key: &str) -> String {
    fields.get(key).cloned().unwrap_or_default()
}
