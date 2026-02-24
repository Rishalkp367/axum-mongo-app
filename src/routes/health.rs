use axum::{extract::State, http::StatusCode, Json};
use bson::doc;
use mongodb::Database;
use serde_json::json;

use crate::app_state::AppState;

pub async fn liveness() -> (StatusCode, Json<serde_json::Value>) {
    tracing::debug!("🔍 Liveness check requested");
    (StatusCode::OK, Json(json!({ "status": "alive" })))
}

pub async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    tracing::debug!("🔍 Readiness check requested");

    let db: &Database = &state.db;
    let ok = db.run_command(doc! { "ping": 1 }, None).await.is_ok();

    if ok {
        tracing::debug!("✅ Readiness check passed: DB is connected");
        (
            StatusCode::OK,
            Json(json!({
                "status": "ready",
                "db": "connected"
            })),
        )
    } else {
        tracing::warn!("⚠️ Readiness check failed: DB is unreachable");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "status": "not_ready",
                "db": "down"
            })),
        )
    }
}
