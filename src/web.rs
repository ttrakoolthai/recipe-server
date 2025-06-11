use crate::*;
use axum::response::Html;

/// Query parameters for retrieving a recipe.
///
/// - `id`: Optional ID of the recipe to load directly.
/// - `tags`: Optional comma-separated list of tags used to filter recipes.
#[derive(Deserialize)]
pub struct GetRecipeParams {
    pub id: Option<String>,
    pub tags: Option<String>,
}

/// Handles the `GET /` route and renders an HTML recipe page.
///
/// This handler checks for query parameters:
/// - If `id` is provided, it fetches the specific recipe and renders it.
/// - If `tags` are provided, it tries to find a matching recipe and redirects to it.
/// - If neither is provided, a random recipe is selected.
///
/// This route uses an Askama template to render the HTML response.
///
/// # Errors
/// Returns a `404 Not Found` if the recipe with the given ID does not exist.
pub async fn get_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<GetRecipeParams>,
) -> Result<response::Response, http::StatusCode> {
    let mut app_writer = app_state.write().await;
    let db = app_writer.db.clone();

    // Fetch recipe by ID if specified
    if let GetRecipeParams { id: Some(id), .. } = params {
        let recipe_result = recipe::get(&db, &id).await;
        let result = match recipe_result {
            Ok((recipe, tags)) => {
                let tag_string = tags.join(", ");
                app_writer.current_recipe = recipe.clone();
                let recipe = IndexTemplate::new(recipe.clone(), tag_string);
                Ok(response::Html(recipe.to_string()).into_response())
            }
            Err(e) => {
                log::warn!("Recipe fetch failed: {}", e);
                Err(http::StatusCode::NOT_FOUND)
            }
        };
        return result;
    }

    // Fetch recipe by tags if specified
    if let GetRecipeParams {
        tags: Some(tags), ..
    } = params
    {
        log::info!("Recipe tags: {}", tags);

        let mut tags_string = String::new();
        for c in tags.chars() {
            if c.is_alphabetic() || c == ',' {
                let cl: String = c.to_lowercase().collect();
                tags_string.push_str(&cl);
            }
        }

        let recipe_result = recipe::get_tagged(&db, tags_string.split(',')).await;
        match recipe_result {
            Ok(Some(id)) => {
                let uri = format!("/?id={}", id);
                return Ok(response::Redirect::to(&uri).into_response());
            }
            Ok(None) => {
                log::info!("Tagged recipe selection was empty");
            }
            Err(e) => {
                log::error!("Tagged recipe selection database error: {}", e);
                panic!("Tagged recipe selection database error");
            }
        }
    }

    // Otherwise, fallback to a random recipe
    let recipe_result = recipe::get_random(&db).await;
    match recipe_result {
        Ok(id) => {
            let uri = format!("/?id={}", id);
            Ok(response::Redirect::to(&uri).into_response())
        }
        Err(e) => {
            log::error!("Random recipe selection failed: {}", e);
            let tag_string = "Empty".to_string();
            let recipe = app_writer.current_recipe.clone();
            let recipe = IndexTemplate::new(recipe, tag_string);
            Ok(Html(recipe.to_string()).into_response())
        }
    }
}

use crate::recipe::Recipe;
use crate::templates::IndexTemplate;

/// Serves a static HTML page with a placeholder recipe.
/// This is used to bootstrap the Leptos frontend UI.
///
/// The recipe shown is hardcoded and not retrieved from the database.
///
/// # Returns
/// An HTML page rendered with Askama.
pub async fn serve_leptos_ui() -> Html<String> {
    let recipe = Recipe {
        id: "placeholder-id".to_string(),
        dish_name: "Sample Dish".to_string(),
        ingredients: "ingredient1, ingredient2".to_string(),
        time_to_prepare: "30 minutes".to_string(),
        source: "https://example.com".to_string(),
    };

    let tags = String::from("sample,example");

    let template = IndexTemplate::new(recipe, tags);
    Html(template.to_string())
}
