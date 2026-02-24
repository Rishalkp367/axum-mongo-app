use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::{
    app_state::AppState,
    models::user_model::{CreateUserRequest, UpdateUserRequest, User},
    repositories::user_repository::UserRepository,
};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses((status = 200, body = User))
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.create(payload).await {
        Ok(user) => {
            tracing::info!(
                user_id = ?user.id,
                name = %user.name,
                "✅ User created successfully"
            );
            Json(json!(user))
        }
        Err(err) => {
            tracing::error!(error = %err, "❌ Failed to create user");
            Json(json!({ "error": err.to_string() }))
        }
    }
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, body = [User])
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, Json<serde_json::Value>> {
    let repo = UserRepository::new(&state.db);

    match repo.find_all().await {
        Ok(users) => {
            tracing::info!(count = users.len(), "✅ Fetched user list");
            Ok(Json(users))
        }
        Err(err) => {
            tracing::error!(error = %err, "❌ Failed to fetch users");
            Err(Json(json!({ "error": err.to_string() })))
        }
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, body = User),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.find_by_id(&id).await {
        Ok(Some(user)) => {
            tracing::info!(user_id = %id, "✅ User found");
            Json(json!(user))
        }
        Ok(None) => {
            tracing::warn!(user_id = %id, "⚠️ User not found");
            Json(json!({ "error": "User not found" }))
        }
        Err(err) => {
            tracing::error!(user_id = %id, error = %err, "❌ Invalid user ID");
            Json(json!({ "error": "Invalid ID" }))
        }
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, body = User),
        (status = 404, description = "User not found"),
        (status = 400, description = "Invalid ID")
    )
)]
pub async fn update_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.update(&id, payload).await {
        Ok(_) => {
            tracing::info!(user_id = %id, "✅ User updated successfully");
            Json(json!({ "status": "updated" }))
        }
        Err(err) => {
            tracing::error!(user_id = %id, error = %err, "❌ Failed to update user");
            Json(json!({ "error": "Update failed" }))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses((status = 200))
)]
pub async fn delete_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.delete(&id).await {
        Ok(_) => {
            tracing::info!(user_id = %id, "✅ User deleted successfully");
            Json(json!({ "status": "deleted" }))
        }
        Err(err) => {
            tracing::error!(user_id = %id, error = %err, "❌ Failed to delete user");
            Json(json!({ "error": "Delete failed" }))
        }
    }
}
