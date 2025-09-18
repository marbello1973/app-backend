use crate::service::all_user_service;

pub async fn root_handler() -> &'static str {
    all_user_service();
    "Cambios de todos los usuarios desde user_handler.rs"
}




/*use UserService::all_users;

use axum::{
    Json,
    extract::{Path, State},
};
use serde_json;
use sqlx::PgPool;

pub async fn list_users_handlers(State(pool): State<PgPool>) -> Json<serde_json::Value> {
    match UserService::all_users(&pool).await {
        Ok(users) => Json(json!({ "users": users })),
        Err(_) => Json(json!({ "error": "Failed to fetch users" })),
    }
}
*/