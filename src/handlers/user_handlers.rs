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
