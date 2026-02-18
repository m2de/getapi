use regex::Regex;
use std::collections::HashMap;

use crate::error::{GetapiError, Result};

pub fn expand(template: &str, vars: &HashMap<String, String>) -> Result<String> {
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    let mut result = template.to_string();
    let mut missing = Vec::new();

    for cap in re.captures_iter(template) {
        let var_name = &cap[1];
        match vars.get(var_name) {
            Some(value) => {
                result = result.replace(&cap[0], value);
            }
            None => {
                missing.push(var_name.to_string());
            }
        }
    }

    if !missing.is_empty() {
        return Err(GetapiError::TemplateVarNotFound(missing.join(", ")));
    }

    Ok(result)
}

/// Like expand, but leaves unresolved `{{var}}` placeholders as `<var>` instead of erroring.
pub fn expand_lenient(template: &str, vars: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    let mut result = template.to_string();

    for cap in re.captures_iter(template) {
        let var_name = &cap[1];
        match vars.get(var_name) {
            Some(value) => {
                result = result.replace(&cap[0], value);
            }
            None => {
                result = result.replace(&cap[0], &format!("<{}>", var_name));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_simple() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "world".to_string());
        assert_eq!(expand("Hello {{name}}!", &vars).unwrap(), "Hello world!");
    }

    #[test]
    fn test_expand_multiple() {
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "1".to_string());
        vars.insert("b".to_string(), "2".to_string());
        assert_eq!(expand("{{a}} and {{b}}", &vars).unwrap(), "1 and 2");
    }

    #[test]
    fn test_expand_missing() {
        let vars = HashMap::new();
        assert!(expand("{{missing}}", &vars).is_err());
    }

    #[test]
    fn test_expand_no_vars() {
        let vars = HashMap::new();
        assert_eq!(expand("no vars here", &vars).unwrap(), "no vars here");
    }
}
