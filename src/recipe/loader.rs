use rust_embed::Embed;
use std::path::Path;

use crate::error::{GetapiError, Result};
use crate::recipe::types::Recipe;

#[derive(Embed)]
#[folder = "providers/"]
struct BundledProviders;

pub fn load_from_file(path: &str) -> Result<Recipe> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(GetapiError::RecipeFileNotFound(path.to_string()));
    }
    let contents = std::fs::read_to_string(p)?;
    let recipe: Recipe =
        serde_json::from_str(&contents).map_err(|e| GetapiError::InvalidRecipe(e.to_string()))?;
    Ok(recipe)
}

pub fn load_all_bundled() -> Vec<Recipe> {
    let mut recipes = Vec::new();
    for filename in BundledProviders::iter() {
        if filename.ends_with(".json") {
            if let Some(file) = BundledProviders::get(&filename) {
                if let Ok(json) = std::str::from_utf8(&file.data) {
                    if let Ok(recipe) = serde_json::from_str::<Recipe>(json) {
                        recipes.push(recipe);
                    }
                }
            }
        }
    }
    recipes
}

pub fn load_from_directory(dir: &str) -> Result<Vec<Recipe>> {
    let path = Path::new(dir);
    if !path.is_dir() {
        return Err(GetapiError::RecipeFileNotFound(format!(
            "Directory not found: {}",
            dir
        )));
    }
    let mut recipes = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) == Some("json") {
            let contents = std::fs::read_to_string(&p)?;
            match serde_json::from_str::<Recipe>(&contents) {
                Ok(recipe) => recipes.push(recipe),
                Err(e) => {
                    eprintln!("Warning: skipping invalid recipe {}: {}", p.display(), e);
                }
            }
        }
    }
    Ok(recipes)
}
