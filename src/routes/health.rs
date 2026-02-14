use axum::{extract::State, Json};
use serde_json::json;
use mongodb::Database;
use bson::doc;

use crate::app_state::AppState;

pub async fn liveness() -> Json<serde_json::Value> {
    Json(json!({
        "status": "alive"
    }))
}

pub async fn readiness(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let db: &Database = &state.db;

    let ok = db.run_command(doc! { "ping": 1 }, None).await.is_ok();

    if ok {
        Json(json!({
            "status": "ready",
            "db": "connected"
        }))
    } else {
        Json(json!({
            "status": "not_ready",
            "db": "down"
        }))
    }
}
