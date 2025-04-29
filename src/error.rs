extern crate serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeServerError {
    #[error("Could not find recipe file: {0}")]
    RecipesNotFound(#[from] std::io::Error),
    #[error("Could not read recipe file: {0}")]
    RecipesMisformat(#[from] serde_json::Error),
    #[error("Invalid database uri: {0}")]
    InvalidDbUri(String),
}
