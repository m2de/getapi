use std::collections::HashMap;

use crate::error::{GetapiError, Result};
use crate::recipe::template;

/// OAuth2 Client Credentials validator.
///
/// Config keys:
/// - `token_url`: The token endpoint URL (required)
/// - `client_id_field`: Name of the collected value to use as client ID (required)
/// - `client_secret_field`: Name of the collected value to use as client secret (required)
/// - `grant_type`: Grant type string (default: "client_credentials")
/// - `scope`: Optional scope parameter
/// - `auth_method`: How to send credentials — "basic" (default) or "body"
pub fn validate(
    values: &HashMap<String, String>,
    config: &HashMap<String, String>,
) -> Result<()> {
    let token_url = config
        .get("token_url")
        .ok_or_else(|| {
            GetapiError::ValidationFailed(
                "oauth2_client_credentials requires 'token_url' in config".to_string(),
            )
        })?;
    let token_url = template::expand(token_url, values)?;

    let client_id_field = config
        .get("client_id_field")
        .ok_or_else(|| {
            GetapiError::ValidationFailed(
                "oauth2_client_credentials requires 'client_id_field' in config".to_string(),
            )
        })?;
    let client_secret_field = config
        .get("client_secret_field")
        .ok_or_else(|| {
            GetapiError::ValidationFailed(
                "oauth2_client_credentials requires 'client_secret_field' in config".to_string(),
            )
        })?;

    let client_id = values
        .get(client_id_field)
        .ok_or_else(|| {
            GetapiError::ValidationFailed(format!("{} not found in collected values", client_id_field))
        })?;
    let client_secret = values
        .get(client_secret_field)
        .ok_or_else(|| {
            GetapiError::ValidationFailed(format!(
                "{} not found in collected values",
                client_secret_field
            ))
        })?;

    let grant_type = config
        .get("grant_type")
        .map(|s| s.as_str())
        .unwrap_or("client_credentials");

    let auth_method = config
        .get("auth_method")
        .map(|s| s.as_str())
        .unwrap_or("basic");

    let client = reqwest::blocking::Client::new();
    let mut req = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded");

    let mut body = format!("grant_type={}", grant_type);
    if let Some(scope) = config.get("scope") {
        body.push_str(&format!("&scope={}", scope));
    }

    if auth_method == "basic" {
        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded = base64_encode(&credentials);
        req = req.header("Authorization", format!("Basic {}", encoded));
    } else {
        body.push_str(&format!(
            "&client_id={}&client_secret={}",
            client_id, client_secret
        ));
    }

    let resp = req.body(body).send()?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        Err(GetapiError::ValidationFailed(format!(
            "OAuth2 token request returned {}: {}",
            status, body
        )))
    }
}

fn base64_encode(input: &str) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARSET[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARSET[((triple >> 12) & 0x3F) as usize] as char);
        if i + 1 < bytes.len() {
            result.push(CHARSET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if i + 2 < bytes.len() {
            result.push(CHARSET[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}
