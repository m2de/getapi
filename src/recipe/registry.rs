use crate::recipe::loader;
use crate::recipe::types::Recipe;

pub struct RecipeRegistry {
    recipes: Vec<Recipe>,
}

impl RecipeRegistry {
    pub fn new() -> Self {
        let recipes = loader::load_all_bundled();
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
