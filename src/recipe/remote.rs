use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{GetapiError, Result};
use crate::recipe::types::Recipe;

const BASE_URL: &str = "https://raw.githubusercontent.com/m2de/getapi/master/providers";

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeIndex {
    pub schema_version: String,
    pub updated_at: String,
    pub recipes: Vec<RecipeIndexEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeIndexEntry {
    pub id: String,
    pub file: String,
    pub version: String,
}

pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("getapi")
        .join("recipes")
}

#[allow(dead_code)]
pub fn is_cache_empty() -> bool {
    let dir = cache_dir();
    if !dir.exists() {
        return true;
    }
    let has_recipes = std::fs::read_dir(&dir)
        .ok()
        .map(|mut entries| {
            entries.any(|e| {
                e.ok()
                    .and_then(|e| {
                        let p = e.path();
                        if p.extension().and_then(|x| x.to_str()) == Some("json") {
                            let name = p.file_name()?.to_str()?.to_string();
                            if name != "index.json" {
                                return Some(true);
                            }
                        }
                        None
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);
    !has_recipes
}

pub fn load_cached_recipes() -> Vec<Recipe> {
    let dir = cache_dir();
    if !dir.exists() {
        return Vec::new();
    }
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut recipes = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("index.json") {
            continue;
        }
        if let Ok(contents) = std::fs::read_to_string(&path) {
            match serde_json::from_str::<Recipe>(&contents) {
                Ok(recipe) => recipes.push(recipe),
                Err(e) => {
                    eprintln!(
                        "Warning: skipping invalid cached recipe {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }
    }
    recipes
}

pub fn fetch_and_cache_all() -> Result<Vec<Recipe>> {
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(format!("getapi/{}", VERSION))
        .build()
        .map_err(|e| GetapiError::RemoteFetch(e.to_string()))?;

    // Fetch index
    let index_url = format!("{}/index.json", BASE_URL);
    let index_resp = client
        .get(&index_url)
        .send()
        .map_err(|e| GetapiError::RemoteFetch(format!("fetching index: {}", e)))?;

    if !index_resp.status().is_success() {
        return Err(GetapiError::RemoteFetch(format!(
            "index request returned {}",
            index_resp.status()
        )));
    }

    let index: RecipeIndex = index_resp
        .json()
        .map_err(|e| GetapiError::RemoteFetch(format!("parsing index: {}", e)))?;

    // Ensure cache dir exists
    let dir = cache_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| GetapiError::RemoteFetch(format!("creating cache dir: {}", e)))?;

    // Save index file
    let index_json = serde_json::to_string_pretty(&index)
        .map_err(|e| GetapiError::RemoteFetch(format!("serialising index: {}", e)))?;
    std::fs::write(dir.join("index.json"), index_json)
        .map_err(|e| GetapiError::RemoteFetch(format!("writing index cache: {}", e)))?;

    // Fetch each recipe
    let mut recipes = Vec::new();
    for entry in &index.recipes {
        let url = format!("{}/{}", BASE_URL, entry.file);
        let resp = client
            .get(&url)
            .send()
            .map_err(|e| GetapiError::RemoteFetch(format!("fetching {}: {}", entry.file, e)))?;

        if !resp.status().is_success() {
            return Err(GetapiError::RemoteFetch(format!(
                "fetching {} returned {}",
                entry.file,
                resp.status()
            )));
        }

        let body = resp
            .text()
            .map_err(|e| GetapiError::RemoteFetch(format!("reading {}: {}", entry.file, e)))?;

        let recipe: Recipe = serde_json::from_str(&body)
            .map_err(|e| GetapiError::RemoteFetch(format!("parsing {}: {}", entry.file, e)))?;

        std::fs::write(dir.join(&entry.file), &body)
            .map_err(|e| GetapiError::RemoteFetch(format!("caching {}: {}", entry.file, e)))?;

        recipes.push(recipe);
    }

    // Write last-check marker
    let now = chrono::Utc::now().to_rfc3339();
    let _ = std::fs::write(dir.join(".last_check"), now);

    Ok(recipes)
}

#[allow(dead_code)]
pub fn clear_cache() -> Result<()> {
    let dir = cache_dir();
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    Ok(())
}
