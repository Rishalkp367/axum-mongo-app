use axum::{ extract::{ State, Path }, Json };
use serde_json::json;

use crate::{
    app_state::AppState,
    repositories::user_repository::UserRepository,
    models::user_model::{ CreateUserRequest, UpdateUserRequest },
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.create(payload).await {
        Ok(user) => Json(json!(user)),
        Err(_) => Json(json!({ "error": "Failed to create user" })),
    }
}

pub async fn list_users(State(state): State<AppState>) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.find_all().await {
        Ok(users) => Json(json!(users)),
        Err(_) => Json(json!({ "error": "Failed to fetch users" })),
    }
}

pub async fn get_user(
    Path(id): Path<String>,
    State(state): State<AppState>
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.find_by_id(&id).await {
        Ok(Some(user)) => Json(json!(user)),
        Ok(None) => Json(json!({ "error": "User not found" })),
        Err(_) => Json(json!({ "error": "Invalid ID" })),
    }
}

pub async fn update_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.update(&id, payload).await {
        Ok(_) => Json(json!({ "status": "updated" })),
        Err(_) => Json(json!({ "error": "Update failed" })),
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(state): State<AppState>
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.delete(&id).await {
        Ok(_) => Json(json!({ "status": "deleted" })),
        Err(_) => Json(json!({ "error": "Delete failed" })),
    }
}
