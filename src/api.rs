use crate::*;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "recipe-server", description = "Recipe-Server API")
    )
)]
pub struct ApiDoc;

pub fn router() -> OpenApiRouter<Arc<RwLock<AppState>>> {
    OpenApiRouter::new()
        .routes(routes!(get_recipe))
        .routes(routes!(get_tagged_recipe))
        .routes(routes!(get_random_recipe))
        .routes(routes!(register))
        .routes(routes!(add_recipe))
}

async fn get_recipe_by_id(db: &SqlitePool, recipe_id: &str) -> Result<response::Response, http::StatusCode> {
    let recipe_result = recipe::get(db, recipe_id).await;
    match recipe_result {
        Ok((recipe, tags)) => Ok(JsonRecipe::new(recipe, tags).into_response()),
        Err(e) => {
            log::warn!("Recipe fetch failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}

#[utoipa::path(
    get,
    path = "/recipe/{recipe_id}",
    responses(
        (status = 200, description = "Get a recipe by id", body = [JsonRecipe]),
        (status = 404, description = "No matching recipe"),
    )
)]
pub async fn get_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(recipe_id): Path<String>,
) -> Result<response::Response, http::StatusCode> {
    let app_reader = app_state.read().await;
    let db = &app_reader.db;
    get_recipe_by_id(db, &recipe_id).await
}

#[utoipa::path(
    get,
    path = "/tagged-recipe",
    responses(
        (status = 200, description = "Get a recipe by tags", body = [JsonRecipe]),
        (status = 404, description = "No matching recipes"),
    )
)]
pub async fn get_tagged_recipe(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(tags): Json<Vec<String>>,
) -> Result<response::Response, http::StatusCode> {
    log::info!("Get tagged recipe: {:?}", tags);
    let app_reader = app_state.read().await;
    let db = &app_reader.db;
    let recipe_result = recipe::get_tagged(db, tags.iter().map(String::as_ref)).await;
    match recipe_result {
        Ok(Some(recipe_id)) => get_recipe_by_id(db, &recipe_id).await,
        Ok(None) => {
            log::warn!("Recipe tag fetch failed tagging");
            Err(http::StatusCode::NOT_FOUND)
        }
        Err(e) => {
            log::warn!("Recipe tag fetch failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}

#[utoipa::path(
    get,
    path = "/random-recipe",
    responses(
        (status = 200, description = "Get a random recipe", body = [JsonRecipe]),
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
        Ok(recipe_id) => get_recipe_by_id(db, &recipe_id).await,
        Err(e) => {
            log::warn!("Get random recipe failed: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}

#[utoipa::path(
    post,
    path = "/register",
    request_body(
        content = inline(authjwt::Registration),
        description = "Get an API key",
    ),
    responses(
        (status = 200, description = "JSON Web Token", body = authjwt::AuthBody),
        (status = 401, description = "Registration failed", body = authjwt::AuthError),
    )
)]
pub async fn register(
    State(appstate): State<SharedAppState>,
    Json(registration): Json<authjwt::Registration>,
) -> axum::response::Response {
    let appstate = appstate.read().await;
    match authjwt::make_jwt_token(&appstate, &registration) {
        Err(e) => e.into_response(),
        Ok(token) => (StatusCode::OK, token).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/add-recipe",
    request_body(
        content = inline(JsonRecipe),
        description = "Recipe to add"
    ),
    responses(
        (status = 201, description = "Added recipe", body = ()),
        (status = 400, description = "Bad request", body = String),
        (status = 401, description = "Auth Error", body = authjwt::AuthError),
    )
)]
pub async fn add_recipe(
    _claims: authjwt::Claims,
    State(appstate): State<SharedAppState>,
    Json(recipe): Json<JsonRecipe>,
) -> axum::response::Response {
    let appstate = appstate.read().await;
    match recipe::add(&appstate.db, recipe).await {
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        Ok(()) => StatusCode::CREATED.into_response(),
    }
}
