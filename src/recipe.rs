use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;

use crate::RecipeServerError;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonRecipe {
    id: String,
    dish_name: String,
    ingredients: String,
    time_to_prepare: String,
    source: String,
    #[serde(default)]
    tags: HashSet<String>,
}

pub struct Recipe {
    pub id: String,
    pub dish_name: String,
    pub ingredients: String,
    pub time_to_prepare: String,
    pub source: String,
}

pub fn read_recipes<P: AsRef<Path>>(recipe_path: P) -> Result<Vec<JsonRecipe>, RecipeServerError> {
    let f = std::fs::File::open(recipe_path.as_ref())?;
    let recipes = serde_json::from_reader(f)?;
    Ok(recipes)
}

impl JsonRecipe {
    pub fn to_recipe(&self) -> (Recipe, impl Iterator<Item = &str>) {
        let recipe = Recipe {
            id: self.id.clone(),
            dish_name: self.dish_name.clone(),
            ingredients: self.ingredients.clone(),
            time_to_prepare: self.time_to_prepare.clone(),
            source: self.source.clone(),
        };
        let tags = self.tags.iter().map(String::deref);
        (recipe, tags)
    }
}
