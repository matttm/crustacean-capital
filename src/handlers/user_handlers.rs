use crate::models;
use crate::services;
use axum::{Json, extract::State};
use sqlx::SqlitePool;

#[axum::debug_handler]
pub async fn get_users(State(pool): State<SqlitePool>) -> Json<Vec<models::user::User>> {
    tracing::info!("Invocation to `get_users`");
    let res = services::user_service::get_users(&pool).await;
    Json(res.unwrap())
}
#[axum::debug_handler]
pub async fn create_user(
    State(pool): State<SqlitePool>,
    user: Json<models::user::UserCreation>,
) -> Json<models::user::User> {
    tracing::info!("Invocation to `create_user`");
    let res = services::user_service::create_user(&pool, user.0).await;
    Json(res.unwrap())
}
