use std::collections::HashMap;

use crate::error::Result;

pub fn write(values: &HashMap<String, String>) -> Result<()> {
    let mut keys: Vec<&String> = values.keys().collect();
    keys.sort();

    for key in keys {
        println!("{}={}", key, values[key]);
    }
    Ok(())
}
