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
        Ok(user) => Json(json!(user)),
        Err(err) => {
            eprintln!("❌ Create user failed: {:?}", err);
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
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            eprintln!("❌ Fetch users failed: {:?}", err);
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
        Ok(Some(user)) => Json(json!(user)),
        Ok(None) => Json(json!({ "error": "User not found" })),
        Err(_) => Json(json!({ "error": "Invalid ID" })),
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
        Ok(_) => Json(json!({ "status": "updated" })),
        Err(_) => Json(json!({ "error": "Update failed" })),
    }
}

#[utoipa::path(delete, path = "/users/{id}",params(
        ("id" = String, Path, description = "User ID")
    ), responses((status = 200)))]
pub async fn delete_user(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let repo = UserRepository::new(&state.db);

    match repo.delete(&id).await {
        Ok(_) => Json(json!({ "status": "deleted" })),
        Err(_) => Json(json!({ "error": "Delete failed" })),
    }
}
