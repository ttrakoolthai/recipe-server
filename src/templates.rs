use crate::*;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    recipe: &'a Recipe,
    stylesheet: &'static str,
    tags: &'a Vec<String>,
}

impl<'a> IndexTemplate<'a> {
    pub fn recipe(recipe: &'a Recipe, tags: &'a Vec<String>) -> Self {
        Self {
            recipe,
            tags,
            stylesheet: "/recipe-server.css",
        }
    }
}
