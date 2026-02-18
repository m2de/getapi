use std::path::Path;

use crate::error::{GetapiError, Result};
use crate::manifest::types::Manifest;

const MANIFEST_PATH: &str = ".getapi/manifest.json";

pub fn load() -> Result<Option<Manifest>> {
    let path = Path::new(MANIFEST_PATH);
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(path)?;
    let manifest: Manifest = serde_json::from_str(&contents)
        .map_err(|e| GetapiError::InvalidRecipe(format!("Invalid manifest: {}", e)))?;
    Ok(Some(manifest))
}
