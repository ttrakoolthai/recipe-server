extern crate serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeServerError {
    #[error("could not find recipe file: {0}")]
    RecipesNotFound(#[from] std::io::Error),
    #[error("could not read recipe file: {0}")]
    RecipeMisformat(#[from] serde_json::Error),
    #[error("invalid database uri: {0}")]
    InvalidDbUri(String),
}
