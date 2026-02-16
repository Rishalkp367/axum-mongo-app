mod app_state;
mod config;
mod db;
mod docs;
mod models;
mod repositories;
mod routes;

use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use app_state::AppState;
use config::Config;
use db::init_db;

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("🛑 Ctrl+C received");
        }
        _ = terminate => {
            println!("🛑 SIGTERM received");
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();

    println!("📡 Connecting to MongoDB...");
    let database = init_db(&config.mongodb_uri, &config.mongodb_db)
        .await
        .expect("❌ Mongo connection failed");

    println!("✅ Connected to MongoDB");

    let state = AppState { db: database };

    let app = routes::create_routes(state)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", docs::ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    println!("🚀 Server running at http://{}", addr);
    println!("📚 Swagger UI available at http://{}/docs", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
