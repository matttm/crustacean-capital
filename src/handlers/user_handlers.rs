use crate::models;
use crate::services;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_users(State(db): State<Db>) -> Json<Vec<models::user::User>> {
    tracing::info!("Invocation to `get_users`");
    let res = services::user_service::get_users(db).await;
    Json(res.unwrap())
}
pub async fn create_user(
    State(db): State<Db>,
    user: Json<models::user::UserCreation>,
) -> Json<models::user::UserCreation> {
    tracing::info!("Invocation to `create_user`");
    let res = services::user_service::create_user(db, user.0).await;
    Json(res.unwrap())
}
