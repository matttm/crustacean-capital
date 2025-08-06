use crate::models;
use sqlx::SqlitePool;

pub async fn get_users(
    pool: &SqlitePool,
) -> Result<Vec<models::user::User>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_users`");
    let users: Vec<models::user::User> =
        sqlx::query_as("SELECT id, username, created_at, updated_at FROM USERS;")
            .fetch_all(pool)
            .await?;
    Ok(users)
}
pub async fn get_user(
    pool: &SqlitePool,
    id: i64,
) -> Result<models::user::User, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_user`");
    let user: models::user::User =
        sqlx::query_as("SELECT id, username, created_at, updated_at FROM USERS WHERE id = ?;")
            .bind(&id)
            .fetch_one(pool)
            .await?;
    Ok(user)
}
pub async fn create_user(
    pool: &SqlitePool,
    user: models::user::UserCreation,
) -> Result<models::user::User, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_user`");
    if user.username.is_empty() || user.password.is_empty() {
        return Err("Missing required fields".into());
    }
    let res = sqlx::query("INSERT INTO USERS (username, password) VALUES (?, ?);")
        .bind(user.username.as_str())
        .bind(user.password.as_str())
        .execute(pool)
        .await?;

    let created = get_user(pool, res.last_insert_rowid()).await?;
    Ok(created)
}

#[cfg(test)]
mod tests {
    use crate::queries;

    use super::*;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(queries::CREATE_TABLE_USER)
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user_and_get_users() {
        let pool = setup_pool().await;
        let user = models::user::UserCreation {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        create_user(&pool, user.clone()).await.unwrap();

        let users = get_users(&pool).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, user.username);
    }

    #[tokio::test]
    async fn test_get_users_empty() {
        let pool = setup_pool().await;
        let users = get_users(&pool).await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_create_multiple_users_and_get_users() {
        let pool = setup_pool().await;
        let user1 = models::user::UserCreation {
            username: "alice".to_string(),
            password: "alicepass".to_string(),
        };
        let user2 = models::user::UserCreation {
            username: "bob".to_string(),
            password: "bobpass".to_string(),
        };
        create_user(&pool, user1.clone()).await.unwrap();
        create_user(&pool, user2.clone()).await.unwrap();

        let users = get_users(&pool).await.unwrap();
        assert_eq!(users.len(), 2);
        let usernames: Vec<String> = users.iter().map(|u| u.username.clone()).collect();
        assert!(usernames.contains(&user1.username));
        assert!(usernames.contains(&user2.username));
    }
    #[tokio::test]
    async fn test_create_user_with_duplicate_username() {
        let pool = setup_pool().await;
        let user = models::user::UserCreation {
            username: "duplicate".to_string(),
            password: "pass1".to_string(),
        };
        create_user(&pool, user.clone()).await.unwrap();

        // Try to create another user with the same username
        let duplicate_user = models::user::UserCreation {
            username: "duplicate".to_string(),
            password: "pass2".to_string(),
        };
        let result = create_user(&pool, duplicate_user.clone()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_with_empty_username_and_password() {
        let pool = setup_pool().await;
        let user = models::user::UserCreation {
            username: "".to_string(),
            password: "".to_string(),
        };
        let result = create_user(&pool, user.clone()).await;
        // Should fail because username and password are NOT NULL
        assert!(result.is_err());
    }
}
