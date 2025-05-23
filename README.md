# geneos-toolkit-rs

[![crates.io](https://img.shields.io/crates/v/geneos-toolkit.svg)](https://crates.io/crates/geneos-toolkit)
[![Documentation](https://docs.rs/geneos-toolkit/badge.svg)](https://docs.rs/geneos-toolkit)
[![Apache-2.0 licensed](https://img.shields.io/crates/l/geneos-toolkit.svg)](./LICENSE)

**geneos-toolkit** is a Rust library for building Geneos Toolkit compatible
applications. It provides utilities for creating structured Geneos Dataviews,
handling environment variables (including encrypted ones), to simplify
integration development.

## Features

- **Dataviews:** Build and format Geneos Dataviews.
- **Secure Environment Variables:** Retrieve and decrypt environment variables
  seamlessly.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
geneos-toolkit = "0.1"  # Use the latest version available
```

## Usage

- Uses the Builder pattern for easy instance initiation.
- The row header is mandatory, set with `set_row_header`.
- Headlines are optional and can be added with `add_headline`.
- At least one value needs to be added to the Dataview, add with `add_value`.
- Values are added in the format `row`, `column`, `value`.
- Rows and Columns are implied in the values and ordered in the Dataview in the
  order they were first introduced.
- Headlines are ordered by the order in which they were added to the Dataview.
- Environment variables can be retrieved with `get_var` or `get_secure_var`.
- Secure variables require a key file path.

### Basic Example Dataview

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> ! {
    let clear_env_var = get_var_or("CLEAR_ENV_VAR", "Default Value");
    let secure_env_var =
        get_secure_var("SECURE_ENV_VAR", "/path/to/key_file").unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1)
        });

    let dataview = Dataview::builder()
        .set_row_header("Process")
        .add_headline(
            "Hostname",
            hostname::get().unwrap_or_default().to_string_lossy(),
        )
        .add_headline("Timestamp", chrono::Utc::now().to_rfc3339())
        .add_headline("Clear Env Var", &clear_env_var)
        .add_headline("Secure Env Var", &secure_env_var)
        .add_value("process1", "Status", "Running")
        .add_value("process1", "CPU", "2.5%")
        .add_value("process1", "Memory", "150MB")
        .build();

    print_result_and_exit(dataview)
}
```

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
