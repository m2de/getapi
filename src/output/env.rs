use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::Result;

pub fn write(path: &str, values: &HashMap<String, String>) -> Result<()> {
    let mut existing = read_existing(path);

    // Merge new values (overwrite existing keys)
    for (key, value) in values {
        existing.insert(key.clone(), value.clone());
    }

    let mut content = String::new();
    let mut keys: Vec<&String> = existing.keys().collect();
    keys.sort();

    for key in keys {
        let value = &existing[key];
        // Quote values that contain spaces or special characters
        if value.contains(' ') || value.contains('#') || value.contains('"') {
            content.push_str(&format!("{}=\"{}\"\n", key, value.replace('"', "\\\"")));
        } else {
            content.push_str(&format!("{}={}\n", key, value));
        }
    }

    fs::write(path, content)?;
    Ok(())
}

pub fn read_existing(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let p = Path::new(path);
    if !p.exists() {
        return map;
    }

    if let Ok(contents) = fs::read_to_string(p) {
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let mut value = value.trim().to_string();
                // Remove surrounding quotes
                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value = value[1..value.len() - 1].to_string();
                }
                map.insert(key, value);
            }
        }
    }

    map
}
