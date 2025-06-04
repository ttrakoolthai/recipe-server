use crate::*;

use std::collections::HashSet;
use std::ops::Deref;
use std::path::Path;

use crate::RecipeServerError;

use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JsonRecipe {
    id: String,
    whos_there: String,
    answer_who: String,
    tags: HashSet<String>,
    source: String,
}

#[derive(Clone)]
pub struct Recipe {
    pub id: String,
    pub whos_there: String,
    pub answer_who: String,
    pub recipe_source: String,
}

pub fn read_recipes<P: AsRef<Path>>(recipes_path: P) -> Result<Vec<JsonRecipe>, RecipeServerError> {
    let f = std::fs::File::open(recipes_path.as_ref())?;
    let recipes = serde_json::from_reader(f)?;
    Ok(recipes)
}

impl JsonRecipe {
    pub fn new(recipe: Recipe, tags: Vec<String>) -> Self {
        let tags = tags.into_iter().collect();
        Self {
            id: recipe.id,
            whos_there: recipe.whos_there,
            answer_who: recipe.answer_who,
            tags,
            source: recipe.recipe_source,
        }
    }

    pub fn to_recipe(&self) -> (Recipe, impl Iterator<Item = &str>) {
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

impl axum::response::IntoResponse for &JsonRecipe {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::OK, axum::Json(&self)).into_response()
    }
}

pub async fn get(db: &SqlitePool, recipe_id: &str) -> Result<(Recipe, Vec<String>), sqlx::Error> {
    let recipe = sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE id = $1;", recipe_id)
        .fetch_one(db)
        .await?;

    let tags: Vec<String> = sqlx::query_scalar!("SELECT tag FROM tags WHERE recipe_id = $1;", recipe_id)
        .fetch_all(db)
        .await?;

    Ok((recipe, tags))
}

pub async fn get_tagged<'a, I>(db: &SqlitePool, tags: I) -> Result<Option<String>, sqlx::Error>
    where I: Iterator<Item=&'a str>
{
    let mut jtx = db.begin().await?;
    sqlx::query("DROP TABLE IF EXISTS qtags;").execute(&mut *jtx).await?;
    sqlx::query("CREATE TEMPORARY TABLE qtags (tag VARCHR(200));")
        .execute(&mut *jtx)
        .await?;
    for tag in tags {
        sqlx::query("INSERT INTO qtags VALUES ($1);")
            .bind(tag)
            .execute(&mut *jtx)
            .await?;
    }
    let recipe_ids = sqlx::query("SELECT DISTINCT recipe_id FROM tags JOIN qtags ON tags.tag = qtags.tag ORDER BY RANDOM() LIMIT 1;")
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

pub async fn get_random(db: &SqlitePool) -> Result<String, sqlx::Error> {
    sqlx::query_scalar!("SELECT id FROM recipes ORDER BY RANDOM() LIMIT 1;")
        .fetch_one(db)
        .await
}
