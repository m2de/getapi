use std::collections::HashMap;
use std::fs;

use crate::error::Result;

pub fn write(path: &str, values: &HashMap<String, String>) -> Result<()> {
    let json = serde_json::to_string_pretty(values)?;
    fs::write(path, json)?;
    Ok(())
}
