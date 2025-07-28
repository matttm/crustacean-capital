use crate::models;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_users(State(db): State<Db>) -> Json<Vec<models::user::User>> {
    tracing::info!("Invocation to `get_users`");
    let mut conn = db.lock().unwrap();
    conn.execute("SELECT id FROM USERS;", ()).unwrap();
    let res: Vec<models::user::User> = vec![];
    Json(res)
}
