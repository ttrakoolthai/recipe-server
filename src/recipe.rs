use crate::RecipeServerError;
use crate::*;

use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;

use serde::Deserialize;
use utoipa::ToSchema;

/// JSON representation of a recipe used for API responses and requests.
///
/// This structure includes metadata such as `tags` which are stored
/// in a set to avoid duplicates.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JsonRecipe {
    id: String,
    dish_name: String,
    ingredients: String,
    time_to_prepare: String,
    tags: HashSet<String>,
    source: String,
}

/// Internal application representation of a recipe.
///
/// This struct is used within the application logic and database operations.
#[derive(Clone)]
pub struct Recipe {
    pub id: String,
    pub dish_name: String,
    pub ingredients: String,
    pub time_to_prepare: String,
    pub source: String,
}

/// Reads and parses a list of recipes from a JSON file.
pub fn read_recipes<P: AsRef<Path>>(recipes_path: P) -> Result<Vec<JsonRecipe>, RecipeServerError> {
    let f = std::fs::File::open(recipes_path.as_ref())?;
    let recipes = serde_json::from_reader(f)?;
    Ok(recipes)
}

impl JsonRecipe {
    /// Constructs a `JsonRecipe` from a `Recipe` and a list of tags.
    pub fn new(recipe: Recipe, tags: Vec<String>) -> Self {
        let tags = tags.into_iter().collect();
        Self {
            id: recipe.id,
            dish_name: recipe.dish_name,
            ingredients: recipe.ingredients,
            time_to_prepare: recipe.time_to_prepare,
            tags,
            source: recipe.source,
        }
    }

    /// Converts a `JsonRecipe` to a `Recipe` and an iterator of tag strings.
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

impl axum::response::IntoResponse for &JsonRecipe {
    /// Converts the `JsonRecipe` into an HTTP response with JSON content.
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::OK, axum::Json(&self)).into_response()
    }
}

/// Fetch a recipe and its tags by ID from the database.
pub async fn get(db: &SqlitePool, recipe_id: &str) -> Result<(Recipe, Vec<String>), sqlx::Error> {
    let recipe = sqlx::query_as!(
        Recipe,
        r#"
        SELECT id, dish_name, ingredients, time_to_prepare, source
        FROM recipes
        WHERE id = $1;
        "#,
        recipe_id
    )
    .fetch_one(db)
    .await?;

    let tags: Vec<String> = sqlx::query_scalar!(
        "SELECT tag FROM recipe_tags WHERE recipe_id = $1;",
        recipe_id
    )
    .fetch_all(db)
    .await?;

    Ok((recipe, tags))
}

/// Fetch the ID of a random recipe with matching tags from the database.
pub async fn get_tagged<'a, I>(db: &SqlitePool, tags: I) -> Result<Option<String>, sqlx::Error>
where
    I: Iterator<Item = &'a str>,
{
    let mut jtx = db.begin().await?;
    sqlx::query("DROP TABLE IF EXISTS qtags;")
        .execute(&mut *jtx)
        .await?;
    sqlx::query("CREATE TEMPORARY TABLE qtags (tag VARCHR(200));")
        .execute(&mut *jtx)
        .await?;
    for tag in tags {
        sqlx::query("INSERT INTO qtags VALUES ($1);")
            .bind(tag)
            .execute(&mut *jtx)
            .await?;
    }
    let recipe_ids = sqlx::query(
        "SELECT DISTINCT recipe_id FROM recipe_tags JOIN qtags ON recipe_tags.tag = qtags.tag ORDER BY RANDOM() LIMIT 1;"
    )
    .fetch_all(&mut *jtx)
    .await?;
    let nrecipe_ids = recipe_ids.len();
    let result = if nrecipe_ids == 1 {
        Some(recipe_ids[0].get(0))
    } else {
        None
    };
    jtx.commit().await?;

    Ok(result)
}

/// Fetch the ID of a random recipe from the database.
pub async fn get_random(db: &SqlitePool) -> Result<String, sqlx::Error> {
    sqlx::query_scalar!("SELECT id FROM recipes ORDER BY RANDOM() LIMIT 1;")
        .fetch_one(db)
        .await
}

/// Insert a new recipe and its tags into the database.
pub async fn add(db: &SqlitePool, recipe: JsonRecipe) -> Result<(), sqlx::Error> {
    let mut jtx = db.begin().await?;

    sqlx::query!(
        r#"
        INSERT INTO recipes (id, dish_name, ingredients, time_to_prepare, source)
        VALUES ($1, $2, $3, $4, $5);
        "#,
        recipe.id,
        recipe.dish_name,
        recipe.ingredients,
        recipe.time_to_prepare,
        recipe.source,
    )
    .execute(&mut *jtx)
    .await?;

    for tag in recipe.tags {
        sqlx::query!(
            r#"INSERT INTO recipe_tags (recipe_id, tag) VALUES ($1, $2);"#,
            recipe.id,
            tag,
        )
        .execute(&mut *jtx)
        .await?;
    }

    jtx.commit().await?;
    Ok(())
}
