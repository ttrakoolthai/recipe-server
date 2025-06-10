use crate::*;

#[derive(Deserialize)]
pub struct GetRecipeParams {
    id: Option<String>,
    tags: Option<String>,
}

pub async fn get_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<GetRecipeParams>,
) -> Result<response::Response, http::StatusCode> {
    let mut app_writer = app_state.write().await;
    let db = app_writer.db.clone();

    // Specified.
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

    if let GetRecipeParams { tags: Some(tags), .. } = params {
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
            let recipe= IndexTemplate::new(recipe, tag_string);
            Ok(response::Html(recipe.to_string()).into_response())
        }
    }
}
