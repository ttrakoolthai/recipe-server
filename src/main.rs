mod error;
mod recipe;
mod templates;

use error::*;
use recipe::*;
use templates::*;

extern crate log;
extern crate mime;

use axum::{self, extract::State, response, routing};
use clap::Parser;
extern crate fastrand;
use sqlx::{SqlitePool, migrate::MigrateDatabase, sqlite};
use tokio::{net, sync::RwLock};
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::Arc;

#[derive(Parser)]
struct Args {
    #[arg(short, long, name = "init-from")]
    init_from: Option<std::path::PathBuf>,
    #[arg(short, long, name = "db-uri")]
    db_uri: Option<String>,
}

struct AppState {
    db: SqlitePool,
    current_recipe: Recipe,
}

async fn get_recipe(State(app_state): State<Arc<RwLock<AppState>>>) -> response::Html<String> {
    let mut app_state = app_state.write().await;
    let db = &app_state.db;
    let recipe_result = sqlx::query_as!(Recipe, "SELECT * FROM recipes ORDER BY RANDOM() LIMIT 1;")
        .fetch_one(db)
        .await;
    match recipe_result {
        Ok(recipe) => app_state.current_recipe = recipe,
        Err(e) => log::warn!("Recipe fetch failed: {}", e),
    }
    let recipe = IndexTemplate::recipe(&app_state.current_recipe);
    response::Html(recipe.to_string())
}

fn get_db_uri(db_uri: Option<&str>) -> String {
    if let Some(db_uri) = db_uri {
        db_uri.to_string()
    } else if let Ok(db_uri) = std::env::var("RECIPES_DB_URI") {
        db_uri
    } else {
        "sqlite://db/recipes.db".to_string()
    }
}

fn extract_db_dir(db_uri: &str) -> Result<&str, RecipeServerError> {
    if db_uri.starts_with("sqlite://") && db_uri.ends_with(".db") {
        let start = db_uri.find(':').unwrap() + 3;
        let mut path = &db_uri[start..];
        if let Some(end) = path.rfind('/') {
            path = &path[..end];
        } else {
            path = "";
        }
        Ok(path)
    } else {
        Err(RecipeServerError::InvalidDbUri(db_uri.to_string()))
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let db_uri = get_db_uri(args.db_uri.as_deref());
    if !sqlite::Sqlite::database_exists(&db_uri).await? {
        let db_dir = extract_db_dir(&db_uri)?;
        std::fs::create_dir_all(db_dir)?;
        sqlite::Sqlite::create_database(&db_uri).await?
    }

    let db = SqlitePool::connect(&db_uri).await?;
    sqlx::migrate!().run(&db).await?;

    if let Some(path) = args.init_from {
        let recipes = read_recipes(path)?;
        'next_recipe: for jj in recipes {
            let j = jj.to_recipe();
            let mut jtx = db.begin().await?;

            let recipe_insert = sqlx::query!(
                "INSERT INTO recipes (id, dish_name, ingredients, time_to_prepare, source) VALUES ($1, $2, $3, $4, $5);",
                j.id,
                j.dish_name,
                j.ingredients,
                j.time_to_prepare,
                j.source,
            )
            .execute(&mut *jtx)
            .await;
            if let Err(e) = recipe_insert {
                eprintln!("Error: Recipe insert: {}: {}", j.id, e);
                jtx.rollback().await?;
                continue;
            }

            // for t in &j.ingredients {
            //     let ingredient_insert = sqlx::query!(
            //         "INSERT INTO ingredients (recipe_id, ingredient) VALUES ($1, $2);",
            //         j.id,
            //         t,
            //     )
            //     .execute(&mut *jtx)
            //     .await;
            //     if let Err(e) = ingredient_insert {
            //         eprintln!("Error: Ingredient insert: {} {}: {}", j.id, t, e);
            //         jtx.rollback().await?;
            //         continue 'next_recipe;
            //     }
            // }

            jtx.commit().await?;
        }
        return Ok(());
    }

    let current_recipe = Recipe {
        id: "mojo".to_string(),
        dish_name: "Mojo".to_string(),
        ingredients: ["Mo' jokes, please.".to_string()].into_iter().collect(),
        time_to_prepare: "0 minutes".to_string(),
        source: "Unknown".to_string(),
    };

    let app_state = AppState { db, current_recipe };
    let state = Arc::new(RwLock::new(app_state));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "recipe-server=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();

    let app = axum::Router::new()
        .route("/", routing::get(get_recipe))
        .route_service(
            "/recipe-server.css",
            services::ServeFile::new_with_mime("assets/static/recipe-server.css", &mime::TEXT_CSS_UTF_8),
        )
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .layer(trace_layer)
        .with_state(state);

    let listener = net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = serve().await {
        eprintln!("recipe-server: error: {}", err);
        std::process::exit(1);
    }
}
