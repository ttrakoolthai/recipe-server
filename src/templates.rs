use crate::*;

use askama::Template;

/// Askama template struct for rendering a recipe in HTML format.
///
/// This template is used to generate the `index.html` page using
/// the `Askama` templating engine. It takes a `Recipe` to display,
/// a CSS stylesheet path, and a string of tags.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    /// The recipe data to be displayed.
    recipe: Recipe,

    /// Path to the associated CSS stylesheet.
    stylesheet: &'static str,

    /// A comma-separated list of tags associated with the recipe.
    tags: String,
}

impl IndexTemplate {
    /// Constructs a new `IndexTemplate` instance with the provided
    /// recipe and tags.
    ///
    /// # Arguments
    ///
    /// * `recipe` - The `Recipe` to display.
    /// * `tags` - A comma-separated `String` of tags to include in the template.
    ///
    /// # Returns
    ///
    /// A fully constructed `IndexTemplate` instance.
    pub fn new(recipe: Recipe, tags: String) -> Self {
        Self {
            recipe,
            stylesheet: "/recipe-server.css",
            tags,
        }
    }
}
