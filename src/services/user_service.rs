use crate::models;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_users(db: Db) -> Result<Vec<models::user::User>, rusqlite::Error> {
    tracing::info!("Invocation to `get_users`");
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, username, created_at, updated_at FROM USERS;")?;
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
pub async fn get_user(db: Db, id: i64) -> Result<models::user::User, rusqlite::Error> {
    tracing::info!("Invocation to `get_user`");
    let conn = db.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT id, username, created_at, updated_at FROM USERS WHERE user_id = ?;")?;
    let res = stmt.query_one([&id], |row| {
        Ok(models::user::User {
            id: row.get(0)?,
            username: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    });
    res
}
pub async fn create_user(
    db: Db,
    user: models::user::UserCreation,
) -> Result<models::user::User, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_user`");
    let conn = db.lock().unwrap();
    if user.username.is_empty() || user.password.is_empty() {
        return Err("Missing required fields".into());
    }
    conn.execute(
        "INSERT INTO USERS (username, password) VALUES (?, ?);",
        [user.username.as_str(), user.password.as_str()],
    )?;
    let created = get_user(db.clone(), conn.last_insert_rowid()).await?;
    Ok(created)
}

#[cfg(test)]
mod tests {
    use crate::queries;

    use super::*;

    fn setup_db() -> Db {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute(queries::CREATE_TABLE_USER, []).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_create_user_and_get_users() {
        let db = setup_db();
        let user = models::user::UserCreation {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        create_user(db.clone(), user.clone()).await.unwrap();

        let users = get_users(db.clone()).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, user.username);
    }

    #[tokio::test]
    async fn test_get_users_empty() {
        let db = setup_db();
        let users = get_users(db.clone()).await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_create_multiple_users_and_get_users() {
        let db = setup_db();
        let user1 = models::user::UserCreation {
            username: "alice".to_string(),
            password: "alicepass".to_string(),
        };
        let user2 = models::user::UserCreation {
            username: "bob".to_string(),
            password: "bobpass".to_string(),
        };
        create_user(db.clone(), user1.clone()).await.unwrap();
        create_user(db.clone(), user2.clone()).await.unwrap();

        let users = get_users(db.clone()).await.unwrap();
        assert_eq!(users.len(), 2);
        let usernames: Vec<String> = users.iter().map(|u| u.username.clone()).collect();
        assert!(usernames.contains(&user1.username));
        assert!(usernames.contains(&user2.username));
    }
    #[tokio::test]
    async fn test_create_user_with_duplicate_username() {
        let db = setup_db();
        let user = models::user::UserCreation {
            username: "duplicate".to_string(),
            password: "pass1".to_string(),
        };
        create_user(db.clone(), user.clone()).await.unwrap();

        // Try to create another user with the same username
        let duplicate_user = models::user::UserCreation {
            username: "duplicate".to_string(),
            password: "pass2".to_string(),
        };
        let result = create_user(db.clone(), duplicate_user.clone()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_with_empty_username_and_password() {
        let db = setup_db();
        let user = models::user::UserCreation {
            username: "".to_string(),
            password: "".to_string(),
        };
        let result = create_user(db.clone(), user.clone()).await;
        // Should fail because username and password are NOT NULL
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_and_check_timestamps() {
        let db = setup_db();
        let user = models::user::UserCreation {
            username: "timestamp_user".to_string(),
            password: "ts_pass".to_string(),
        };
        create_user(db.clone(), user.clone()).await.unwrap();

        let conn = db.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT created_at, updated_at FROM USERS WHERE username = ?")
            .unwrap();
        let mut rows = stmt.query([user.username.as_str()]).unwrap();
        let row = rows.next().unwrap().unwrap();
        let created_at: String = row.get(0).unwrap();
        let updated_at: String = row.get(1).unwrap();
        assert!(!created_at.is_empty());
        assert!(!updated_at.is_empty());
    }
}
