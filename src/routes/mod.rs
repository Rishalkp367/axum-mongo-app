pub mod health;
pub mod user_routes;

use axum::{ Router, routing::{ get, post, put, delete } };
use crate::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Health
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))

        // Users CRUD
        .route("/users", post(user_routes::create_user).get(user_routes::list_users))
        .route("/users/:id", get(user_routes::get_user))
        .route("/users/:id", put(user_routes::update_user))
        .route("/users/:id", delete(user_routes::delete_user))

        .with_state(state)
}
