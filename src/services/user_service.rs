use crate::models;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_users(db: Db) -> Result<Vec<models::user::User>, rusqlite::Error> {
    tracing::info!("Invocation to `get_users`");
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, username, created_at, updated_at FROM USERS;")
        .unwrap();
    let mut raw = stmt.query(())?;
    let mut res: Vec<models::user::User> = vec![];
    while let Some(row) = raw.next()? {
        res.push(models::user::User {
            id: row.get(0)?,
            username: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    }
    Ok(res)
}
pub async fn create_user(
    db: Db,
    user: models::user::UserCreation,
) -> Result<models::user::UserCreation, rusqlite::Error> {
    tracing::info!("Invocation to `create_user`");
    let conn = db.lock().unwrap();
    let res = conn.execute(
        "INSERT INTO USERS (username, password) VALUES (?, ?);",
        [user.username.as_str(), user.password.as_str()],
    )?;
    Ok(user)
}
