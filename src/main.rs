mod config;
mod db;
mod app_state;
mod routes;
mod models;
mod repositories;

use tokio::{ net::TcpListener };
use tower_http::{ cors::CorsLayer, trace::TraceLayer };
use tracing_subscriber::EnvFilter;

use config::Config;
use db::init_db;
use app_state::AppState;

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix
            ::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv().await;
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
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = Config::from_env();

    println!("📡 Connecting to Mongo...");
    let database = init_db(&config.mongodb_uri, &config.mongodb_db).await;

    let state = AppState { db: database };

    let app = routes
        ::create_routes(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal()).await
        .expect("Server crashed");
}
