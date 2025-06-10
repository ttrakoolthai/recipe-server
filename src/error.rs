extern crate serde_json;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeServerError {
    #[error("Recipe file not found: {0}")]
    RecipeNotFound(#[from] std::io::Error),

    #[error("Unable to read recipe file: {0}")]
    RecipeMisformat(#[from] serde_json::Error),

    #[error("Invalid database uri: {0}")]
    InvalidDbUri(String),
}
