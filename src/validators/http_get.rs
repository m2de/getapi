use std::collections::HashMap;

use crate::error::{GetapiError, Result};
use crate::recipe::template;

/// Generic HTTP GET validator. Fully driven by config from the recipe.
///
/// Config keys:
/// - `url`: The URL to request (supports {{var}} templates resolved against collected values)
/// - `header_name`: Header name to set (e.g. "Authorization")
/// - `header_value`: Header value template (e.g. "Bearer {{API_KEY}}")
/// - `api_key_field`: Shorthand — sends `Authorization: Bearer <value of this field>`
/// - `headers.<Name>`: Extra headers — any config key starting with `headers.` adds a header
///   with the remainder as the name and the value (template-expanded) as the header value.
///   E.g. `"headers.Notion-Version": "2022-06-28"` adds `Notion-Version: 2022-06-28`.
///
/// If none of header_name/header_value/api_key_field are set, the request is made with no auth.
pub fn validate(
    values: &HashMap<String, String>,
    config: &HashMap<String, String>,
) -> Result<()> {
    let url_template = config
        .get("url")
        .ok_or_else(|| {
            GetapiError::ValidationFailed(
                "http_get validator requires 'url' in config".to_string(),
            )
        })?;

    let url = template::expand(url_template, values)?;

    let client = reqwest::blocking::Client::new();
    let mut req = client.get(&url);

    if let (Some(name), Some(value_template)) =
        (config.get("header_name"), config.get("header_value"))
    {
        let value = template::expand(value_template, values)?;
        req = req.header(name, value);
    } else if let Some(field_name) = config.get("api_key_field") {
        let api_key = values.get(field_name).ok_or_else(|| {
            GetapiError::ValidationFailed(format!("{} not found", field_name))
        })?;
        req = req.header("Authorization", format!("Bearer {}", api_key));
    }

    // Add any extra headers from `headers.*` config keys
    for (key, value_template) in config {
        if let Some(header_name) = key.strip_prefix("headers.") {
            let value = template::expand(value_template, values)?;
            req = req.header(header_name, value);
        }
    }

    let resp = req.send()?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        Err(GetapiError::ValidationFailed(format!(
            "HTTP GET {} returned {}",
            url, status
        )))
    }
}
