use std::collections::HashMap;

use crate::recipe::loader;
use crate::recipe::remote;
use crate::recipe::types::Recipe;

pub struct RecipeRegistry {
    recipes: Vec<Recipe>,
}

impl RecipeRegistry {
    pub fn new() -> Self {
        let bundled = loader::load_all_bundled();
        let cached = remote::load_cached_recipes();
        let recipes = merge_recipes(bundled, cached);
        Self { recipes }
    }

    pub fn with_extra_dir(mut self, dir: &str) -> Self {
        if let Ok(extra) = loader::load_from_directory(dir) {
            self.recipes.extend(extra);
        }
        self
    }

    pub fn find(&self, id: &str) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    pub fn all(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn search(&self, query: &str) -> Vec<&Recipe> {
        let q = query.to_lowercase();
        self.recipes
            .iter()
            .filter(|r| {
                r.id.to_lowercase().contains(&q)
                    || r.display_name.to_lowercase().contains(&q)
                    || r.description.to_lowercase().contains(&q)
                    || r.category.iter().any(|c| c.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn filter_by_category(&self, category: &str) -> Vec<&Recipe> {
        let cat = category.to_lowercase();
        self.recipes
            .iter()
            .filter(|r| r.category.iter().any(|c| c.to_lowercase() == cat))
            .collect()
    }
}

/// Merge two recipe lists. `overlay` takes precedence when the same `id` appears in both.
fn merge_recipes(base: Vec<Recipe>, overlay: Vec<Recipe>) -> Vec<Recipe> {
    let mut map: HashMap<String, Recipe> = HashMap::new();
    for recipe in base {
        map.insert(recipe.id.clone(), recipe);
    }
    for recipe in overlay {
        map.insert(recipe.id.clone(), recipe);
    }
    let mut merged: Vec<Recipe> = map.into_values().collect();
    merged.sort_by(|a, b| a.id.cmp(&b.id));
    merged
}
