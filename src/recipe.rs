use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;

use crate::RecipeServerError;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonRecipe {
    id: String,
    whos_there: String,
    answer_who: String,
    tags: HashSet<String>,
    source: String,
}

pub struct Recipe {
    pub id: String,
    pub whos_there: String,
    pub answer_who: String,
    pub recipe_source: String,
}

pub fn read_recipes<P: AsRef<Path>>(recipe_path: P) -> Result<Vec<JsonRecipe>, RecipeServerError> {
    let f = std::fs::File::open(recipe_path.as_ref())?;
    let recipes = serde_json::from_reader(f)?;
    Ok(recipes)
}

impl JsonRecipe {
    pub fn to_recipe(&self) -> (Recipe, impl Iterator<Item=&str>) {
        let recipe = Recipe {
            id: self.id.clone(),
            whos_there: self.whos_there.clone(),
            answer_who: self.answer_who.clone(),
            recipe_source: self.source.clone(),
        };
        let tags = self.tags.iter().map(String::deref);
        (recipe, tags)
    }
}
