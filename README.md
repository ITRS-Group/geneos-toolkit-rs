# geneos-toolkit-rs

[![crates.io](https://img.shields.io/crates/v/geneos-toolkit.svg)](https://crates.io/crates/geneos-toolkit)
[![Documentation](https://docs.rs/geneos-toolkit/badge.svg)](https://docs.rs/geneos-toolkit)
[![Apache-2.0 licensed](https://img.shields.io/crates/l/geneos-toolkit.svg)](./LICENSE)

**geneos-toolkit** is a Rust library for building Geneos Toolkit compatible
applications. It provides utilities for creating structured Geneos Dataviews,
handling environment variables (including encrypted ones), to simplify
integration development. For the Geneos Toolkit plugin format and lifecycle,
see the Geneos Toolkit docs: https://docs.itrsgroup.com/docs/geneos/current/collection/toolkit-plugin/index.html

## Features

- **Dataviews:** Build and format Geneos Dataviews.
- **Row Builder:** Construct rows via `Row` + `add_row` without repeating the row id.
- **Secure Environment Variables (feature-gated):** Enable `secure-env` to expose secure helpers (`decrypt`, `get_secure_var`, etc.) for encrypted env vars.
- **Lean by default:** With `secure-env` disabled, secure helpers are absent and there are zero third-party runtime dependencies.

## Installation

Add the following to your `Cargo.toml`:

```toml
# Lean default (no secure env, zero third-party runtime dependencies)
[dependencies]
geneos-toolkit = "0.2"

# Enable secure env helpers (adds crypto dependencies)
# geneos-toolkit = { version = "0.2", features = ["secure-env"] }
```

## Usage

- Uses the Builder pattern for easy instance initiation.
- The row header is mandatory, set with `set_row_header`.
- Headlines are optional and can be added with `add_headline`.
- Add values with `add_value(row, column, value)`.
- Add whole rows with `Row::new(...).add_cell(...).add_row(row)`.
- Rows/columns keep insertion order by default; optional sorting is available via
  `sort_rows()`, `sort_rows_by(...)`, or `sort_rows_with(...)`.
- Headlines are ordered by the order in which they were added to the Dataview.
- Environment variables: `get_var`/`get_var_or` always available; secure helpers (`get_secure_var`, `decrypt`) only with `secure-env`.
- Secure variables require a key file path when `secure-env` is enabled.

### Dataview Layout (annotated)

```text
+-------------+-----------------+-----------------+
| row header  | column1         | column2         |  <-- header row (row header + column names)
+=============+=================+=================+
| <!>headline1| value1          |                 |  <-- headline rows (metadata, prefixed with "<!>")
| <!>headline2| value2          |                 |
+-------------+-----------------+-----------------+
| rowA        | valA1           | valA2           |  <-- data rows (row name + cell values)
| rowB        | valB1           | valB2           |
+-------------+-----------------+-----------------+
```

Rendered output for the same layout:

```text
row_header,column1,column2
<!>headline1,value1
<!>headline2,value2
rowA,valA1,valA2
rowB,valB1,valB2
```

### Basic Example Dataview

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clear_env_var = get_var_or("CLEAR_ENV_VAR", "Default Value")?;

    let dataview = Dataview::builder()
        .set_row_header("Process")
        .add_headline(
            "Hostname",
            hostname::get().unwrap_or_default().to_string_lossy(),
        )
        .add_headline("Timestamp", chrono::Utc::now().to_rfc3339())
        .add_headline("Clear Env Var", &clear_env_var)
        .add_value("process1", "Status", "Running")
        .add_value("process1", "CPU", "2.5%")
        .add_value("process1", "Memory", "150MB")
        .build()?;

    println!("{}", dataview);
    Ok(())
}
```

### Row Builder + Optional Sorting

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let row1 = Row::new("server-02")
        .add_cell("cpu", "45%")
        .add_cell("status", "active");
    let row2 = Row::new("server-01")
        .add_cell("cpu", "12%")
        .add_cell("status", "idle");

    let dataview = Dataview::builder()
        .set_row_header("host")
        .add_headline("region", "us-east-1")
        .add_row(row1)
        .add_row(row2)
        .sort_rows() // opt-in: otherwise insertion order is kept
        .build()?;

    println!("{}", dataview);
    Ok(())
}
```

### Iterative Rows + Custom Sort

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Unknown order at runtime; we sort descending by name afterward.
    let hosts = ["beta", "alpha", "gamma"];

    let mut builder = Dataview::builder()
        .set_row_header("host")
        .add_headline("source", "inventory");

    for name in hosts {
        let row = Row::new(name)
            .add_cell("status", "up")
            .add_cell("cpu", "n/a");
        builder = builder.add_row(row);
    }

    let dataview = builder
        .sort_rows_with(|a, b| b.cmp(a)) // custom comparator: reverse lexicographic
        .build()?;

    println!("{}", dataview);
    Ok(())
}
```

### Secure Environment Variables (feature-gated)

Enable the `secure-env` feature to add the secure helpers:

```toml
[dependencies]
geneos-toolkit = { version = "0.2", features = ["secure-env"] }
```

This feature gates `decrypt`, `get_secure_var`, and related helpers.

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = get_secure_var("MY_SECRET", "/path/to/keyfile")?;
    println!("Secret: {}", secret);
    Ok(())
}
```

- Without `secure-env`, encrypted values (`+encs+`) make `get_var`/`get_var_or` return `MissingSecureEnvSupport`, and the secure helpers are not exposed.

## Contributing

Contributions are welcome! If you have suggestions, bug fixes, or enhancements,
please open an issue or submit a pull request.

## License

This project is licensed under the Apache License, Version 2.0.

```text
   Copyright 2025 ITRS Group

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
