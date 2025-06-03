use crate::*;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "recipe-server", description = "Recipe Server API")
    )
)]
pub struct ApiDoc;

pub fn router() -> OpenApiRouter<Arc<RwLock<AppState>>> {
    OpenApiRouter::new()
        .routes(routes!(get_recipe))
        .routes(routes!(get_tagged_recipe))
        .routes(routes!(get_random_recipe))
}

async fn get_recipe_by_id(db: &SqlitePool, joke_id: &str) -> Result<response::Response, http::StatusCode> {
    let recipe_result = recipe::get(db, joke_id).await;
    match recipe_result {
        Ok((joke, tags)) => Ok(JsonJoke::new(joke, tags).into_response()),
        Err(e) => {
            log::warn!("Recipe fetch failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}

#[utoipa::path(
    get,
    path = "/joke/{joke_id}",
    responses(
        (status = 200, description = "Get a joke by id", body = [JsonJoke]),
        (status = 404, description = "No matching joke"),
    )
)]
pub async fn get_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(joke_id): Path<String>,
) -> Result<response::Response, http::StatusCode> {
    let app_reader = app_state.read().await;
    let db = &app_reader.db;
    get_recipe_by_id(db, &joke_id).await
}

#[utoipa::path(
    get,
    path = "/tagged-joke",
    responses(
        (status = 200, description = "Get a joke by tags", body = [JsonJoke]),
        (status = 404, description = "No matching jokes"),
    )
)]
pub async fn get_tagged_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(tags): Json<Vec<String>>,
) -> Result<response::Response, http::StatusCode> {
    log::info!("get tagged joke: {:?}", tags);
    let app_reader = app_state.read().await;
    let db = &app_reader.db;
    let recipe_result = recipe::get_tagged(db, tags.iter().map(String::as_ref)).await;
    match recipe_result {
        Ok(Some(joke_id)) => get_recipe_by_id(db, &joke_id).await,
        Ok(None) => {
            log::warn!("joke tag fetch failed tagging");
            Err(http::StatusCode::NOT_FOUND)
        }
        Err(e) => {
            log::warn!("joke tag fetch failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}

#[utoipa::path(
    get,
    path = "/random-recipe",
    responses(
        (status = 200, description = "Get a random recipe", body = [JsonJoke]),
        (status = 404, description = "No recipe"),
    )
)]
pub async fn get_random_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
) -> Result<response::Response, http::StatusCode> {
    let app_reader = app_state.read().await;
    let db = &app_reader.db;
    let recipe_result = recipe::get_random(db).await;
    match recipe_result {
        Ok(joke_id) => get_recipe_by_id(db, &joke_id).await,
        Err(e) => {
            log::warn!("get random joke failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}
