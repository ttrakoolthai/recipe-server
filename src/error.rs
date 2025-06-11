/// External crate used for handling JSON serialization/deserialization errors.
extern crate serde_json;

use thiserror::Error;

/// Represents possible errors that can occur in the Recipe Server application.
///
/// This enum is used to encapsulate different error types encountered when
/// working with recipe files or the database configuration.
#[derive(Debug, Error)]
pub enum RecipeServerError {
    /// Occurs when a recipe file cannot be found or opened.
    ///
    /// This wraps a standard I/O error, typically from file system operations.
    #[error("Recipe file not found: {0}")]
    RecipeNotFound(#[from] std::io::Error),

    /// Occurs when a recipe file cannot be parsed as valid JSON.
    ///
    /// This wraps a serde_json error, indicating that the file was not
    /// correctly formatted as JSON.
    #[error("Unable to read recipe file: {0}")]
    RecipeMisformat(#[from] serde_json::Error),

    /// Occurs when the provided database URI is invalid.
    ///
    /// This variant contains a string with the problematic URI.
    #[error("Invalid database uri: {0}")]
    InvalidDbUri(String),
}
