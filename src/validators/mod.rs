pub mod http_get;
pub mod oauth2;

use std::collections::HashMap;

use crate::error::{GetapiError, Result};

/// Validator function signature.
/// - `values`: collected credential values (output_key → value)
/// - `config`: arbitrary config from the recipe's validate step
type ValidatorFn = fn(&HashMap<String, String>, &HashMap<String, String>) -> Result<()>;

struct ValidatorEntry {
    name: &'static str,
    func: ValidatorFn,
}

const VALIDATORS: &[ValidatorEntry] = &[
    ValidatorEntry {
        name: "http_get",
        func: http_get::validate,
    },
    ValidatorEntry {
        name: "oauth2_client_credentials",
        func: oauth2::validate,
    },
];

pub fn run(
    method: &str,
    values: &HashMap<String, String>,
    config: &HashMap<String, String>,
) -> Result<()> {
    for entry in VALIDATORS {
        if entry.name == method {
            return (entry.func)(values, config);
        }
    }

    let available: Vec<&str> = VALIDATORS.iter().map(|e| e.name).collect();
    Err(GetapiError::ValidatorNotFound(
        method.to_string(),
        available.join(", "),
    ))
}

