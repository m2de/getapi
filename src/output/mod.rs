pub mod env;
pub mod json;
pub mod stdout;

use std::collections::HashMap;

use crate::cli::args::OutputFormat;
use crate::error::Result;

pub fn write_output(
    format: &OutputFormat,
    file: &str,
    values: &HashMap<String, String>,
) -> Result<()> {
    match format {
        OutputFormat::Env => env::write(file, values),
        OutputFormat::Json => json::write(file, values),
        OutputFormat::Stdout => stdout::write(values),
    }
}
