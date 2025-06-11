use tower_http::services::{ServeDir, ServeFile};

mod api;
mod authjwt;
mod error;
mod recipe;
mod templates;
mod web;

extern crate fastrand;
extern crate log;
extern crate mime;

use axum::{
    self, RequestPartsExt,
    extract::{Json, Query, State},
    http::{self, StatusCode},
    response::{self, IntoResponse},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::{TimeDelta, prelude::*};
use clap::Parser;
use error::*;
use jsonwebtoken::{DecodingKey, EncodingKey};
use recipe::*;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool, migrate::MigrateDatabase, sqlite};
use std::borrow::Cow;
use std::sync::Arc;
use tokio::{net, signal, sync::RwLock, time::Duration};
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

/// Command-line arguments for configuring the server.
#[derive(Parser)]
struct Args {
    /// Optional path to JSON file for initializing database.
    #[arg(long, name = "Init-from")]
    init_from: Option<std::path::PathBuf>,

    /// Optional SQLite database URI.
    #[arg(short, long, name = "db-uri")]
    db_uri: Option<String>,

    /// IP address to bind the server to.
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    /// Port number to bind the server to.
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

/// Shared application state structure.
struct AppState {
    db: SqlitePool,
    jwt_keys: authjwt::JwtKeys,
    reg_key: String,
    current_recipe: Recipe,
}

type SharedAppState = Arc<RwLock<AppState>>;

impl AppState {
    /// Create a new instance of `AppState`.
    pub fn new(db: SqlitePool, jwt_keys: authjwt::JwtKeys, reg_key: String) -> Self {
        let current_recipe = Recipe {
            id: "placeholder-id".to_string(),
            dish_name: "Sample Dish".to_string(),
            ingredients: "example ingredient1, ingredient2".to_string(),
            time_to_prepare: "30 minutes".to_string(),
            source: "https://example.com".to_string(),
        };
        Self {
            db,
            jwt_keys,
            reg_key,
            current_recipe,
        }
    }
}

/// Determine the SQLite database URI to use.
fn get_db_uri(db_uri: Option<&str>) -> Cow<str> {
    if let Some(db_uri) = db_uri {
        db_uri.into()
    } else if let Ok(db_uri) = std::env::var("DATABASE_URL") {
        db_uri.into()
    } else {
        "sqlite://db/recipes.db".into()
    }
}

/// Extract directory path from SQLite URI.
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

/// Handle shutdown signal (SIGINT, SIGTERM) and perform graceful cleanup.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to create SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C (SIGINT) signal.");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal.");
        },
    }

    tracing::info!("Initiating graceful shutdown...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    tracing::info!("Cleanup complete.");
}

/// Launch the web service.
async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let tsf = tracing_subscriber::fmt::layer().with_writer(std::io::stderr);
    let tse = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "recipe_server=debug".into());
    tracing_subscriber::registry().with(tsf).with(tse).init();

    log::info!("Starting...");

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
            let mut jtx = db.begin().await?;
            let (j, ts) = jj.to_recipe();
            let recipe_insert = sqlx::query!(
                "INSERT INTO recipes (id, dish_name, ingredients, time_to_prepare, source)
                 VALUES ($1, $2, $3, $4, $5);",
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
            };
            for t in ts {
                let tag_insert = sqlx::query!(
                    "INSERT INTO recipe_tags (recipe_id, tag) VALUES ($1, $2);",
                    j.id,
                    t,
                )
                .execute(&mut *jtx)
                .await;
                if let Err(e) = tag_insert {
                    eprintln!("Error--Tag insert: {} {}: {}", j.id, t, e);
                    jtx.rollback().await?;
                    continue 'next_recipe;
                };
            }
            jtx.commit().await?;
        }
        return Ok(());
    }

    let jwt_keys = authjwt::make_jwt_keys().await.unwrap_or_else(|_| {
        tracing::error!("jwt keys");
        std::process::exit(1);
    });

    let reg_key = authjwt::read_secret("REG_PASSWORD", "secrets/reg_password.txt")
        .await
        .unwrap_or_else(|_| {
            tracing::error!("Reg password");
            std::process::exit(1);
        });

    let app_state = AppState::new(db, jwt_keys, reg_key);
    let state = Arc::new(RwLock::new(app_state));

    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(tower_http::cors::Any);

    async fn handler_404() -> axum::response::Response {
        (http::StatusCode::NOT_FOUND, "404 Not Found").into_response()
    }

    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();

    let (api_router, api) = OpenApiRouter::with_openapi(api::ApiDoc::openapi())
        .nest("/api/v1", api::router())
        .split_for_parts();

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone());
    let redoc_ui = Redoc::with_url("/redoc", api);
    let rapidoc_ui = RapiDoc::new("/api-docs/openapi.json").path("/rapidoc");

    let app = axum::Router::new()
        .route("/", axum::routing::get(web::get_recipe))
        .nest_service("/pkg", ServeDir::new("leptos_frontend/dist/pkg"))
        .route(
            "/ui",
            axum::routing::get(|| async { web::serve_leptos_ui().await }),
        )
        .route_service(
            "/index.html",
            ServeFile::new("leptos_frontend/dist/index.html"),
        )
        .fallback(handler_404)
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .merge(api_router)
        .fallback(handler_404)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state);

    let endpoint = format!("{}:{}", args.ip, args.port);
    let listener = net::TcpListener::bind(&endpoint).await?;
    log::info!("Started: Listening on {}", endpoint);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Main entry point that launches the server.
#[tokio::main]
async fn main() {
    if let Err(err) = serve().await {
        eprintln!("recipe_server: error: {}", err);
        std::process::exit(1);
    }
}
