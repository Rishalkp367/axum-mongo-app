mod app_state;
mod config;
mod db;
mod docs;
mod models;
mod repositories;
mod routes;

use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
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
            tracing::info!("🛑 Ctrl+C received, shutting down");
        }
        _ = terminate => {
            tracing::info!("🛑 SIGTERM received, shutting down");
        }
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let log_format = std::env::var("LOG_FORMAT").unwrap_or_default();

    if log_format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .with_current_span(true)
            .with_span_list(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(filter)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    }
}

async fn run() {
    let config = Config::from_env();

    tracing::info!("📡 Connecting to MongoDB...");
    let database = init_db(&config.mongodb_uri, &config.mongodb_db)
        .await
        .expect("❌ Mongo connection failed");

    let state = AppState { db: database };

    let app = routes::create_routes(state)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", docs::ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        );

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("🚀 Server running at http://{}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/docs", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn main() {
    // Load .env before building the runtime so WORKER_THREADS is available.
    dotenvy::dotenv().ok();

    init_tracing();

    let mut builder = tokio::runtime::Builder::new_multi_thread();

    if let Some(n) = std::env::var("WORKER_THREADS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    {
        tracing::info!("⚙️  Using {} worker thread(s) from WORKER_THREADS", n);
        builder.worker_threads(n);
    }

    builder
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime")
        .block_on(run());
}
