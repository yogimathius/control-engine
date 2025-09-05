use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

use codex_control_engine::{auth, handlers, CodexEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/codex_sacred".to_string());

    let db = sqlx::PgPool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;

    // Initialize the sacred engine
    let engine = Arc::new(CodexEngine::new()?);

    let app_state = handlers::AppState { db, engine };

    // Build sacred API routes
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/users/register", post(handlers::register_user))
        .route("/api/users/login", post(handlers::login_user))
        .route("/api/users/profile", get(handlers::get_profile)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/rituals/execute", post(handlers::execute_ritual)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/rituals/catalog", get(handlers::get_ritual_catalog))
        .route("/api/rituals/upload", post(handlers::upload_ritual)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/rituals/:id", get(handlers::get_ritual_details))
        .route("/api/state/current", get(handlers::get_current_state)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/state/transform", post(handlers::transform_state)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/state/history", get(handlers::get_state_history)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .route("/api/state/reflection", post(handlers::request_reflection)
            .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Server configuration
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);
    
    let addr = SocketAddr::from((host.parse::<std::net::IpAddr>().unwrap_or([127, 0, 0, 1].into()), port));
    println!("🔮 Codex Sacred Server listening on {}", addr);
    println!("✨ May this technology serve the highest good");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "Sacred systems operational",
        "version": env!("CARGO_PKG_VERSION"),
        "message": "🔮 The Codex Control Engine serves"
    }))
}
