# geneos-toolkit-rs

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

### Basic Example Dataview

```rust,no_run
use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clear_env_var = get_var_or("CLEAR_ENV_VAR", "Default Value");
    let secure_env_var = get_secure_var("SECURE_ENV_VAR", "/path/to/key_file")?;

    let dataview = Dataview::builder()
        .set_row_header("Process")
        .add_headline("Hostname", &hostname::get().unwrap_or_default().to_string_lossy())
        .add_headline("Timestamp", &chrono::Utc::now().to_rfc3339())
        .add_headline("Clear Env Var", &clear_env_var)
        .add_headline("Secure Env Var", &secure_env_var)
        .add_value("process1", "Status", "Running")
        .add_value("process1", "CPU", "2.5%")
        .add_value("process1", "Memory", "150MB")
        .build()?;

    println!("{}", dataview);
    Ok(())
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
