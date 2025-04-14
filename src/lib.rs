/// Geneos Toolkit library for building data samplers and integrations
///
/// This library provides utilities for creating Geneos Dataviews, handling environment variables
/// (including encrypted ones), and other helpers for working with the Geneos Toolkit.
///
/// Rows and columns are ordered by the order in which they are first added to the `Dataview`.
///
/// # Example
///
/// ```no_run,rust
/// use geneos_toolkit::prelude::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let clear_env_var = get_var_or("CLEAR_ENV_VAR", "Default");
///     let secure_env_var = get_secure_var_or("SECURE_ENV_VAR", "/path/to/key_file", "Default")?;
///     
///     let dataview = Dataview::builder()
///         .set_row_header("Process")
///         .add_headline("Hostname", &hostname::get().unwrap_or_default().to_string_lossy())
///         .add_headline("Timestamp", &chrono::Utc::now().to_rfc3339())
///         .add_headline("Clear Env Var", &clear_env_var)
///         .add_headline("Secure Env Var", &secure_env_var)
///         .add_value("process1", "Status", "Running")
///         .add_value("process1", "CPU", "2.5%")
///         .add_value("process1", "Memory", "150MB")
///         .build()?;
///
///     println!("{}", dataview);
///     Ok(())
/// }
/// ```
pub mod dataview;
pub mod env;

pub mod prelude {
    pub use crate::dataview::{print_result_and_exit, Dataview};
    pub use crate::env::{get_secure_var, get_secure_var_or, get_var, get_var_or};
}
