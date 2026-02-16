use utoipa::OpenApi;

use crate::routes::user_routes;
use crate::models::user_model::{
    User,
    CreateUserRequest,
    UpdateUserRequest,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        user_routes::create_user,
        user_routes::list_users,
        user_routes::get_user,
        user_routes::update_user,
        user_routes::delete_user,
    ),
    components(
        schemas(User, CreateUserRequest, UpdateUserRequest)
    ),
    tags(
        (name = "Users", description = "User CRUD operations")
    )
)]
pub struct ApiDoc;
