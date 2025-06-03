use crate::*;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    joke: Recipe,
    stylesheet: &'static str,
    tags: String,
}

impl IndexTemplate {
    pub fn new(joke: Recipe, tags: String) -> Self {
        Self {
            joke,
            stylesheet: "/recipe-server.css",
            tags,
        }
    }
}
