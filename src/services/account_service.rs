use std::str::FromStr;

use crate::models;
use crate::services::{generation_service, user_service};

use sqlx::SqlitePool;

pub async fn get_accounts(
    pool: &SqlitePool,
) -> Result<Vec<models::account::AccountGeneral>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_accounts`");
    let res: Vec<models::account::AccountGeneral> =
        sqlx::query_as("SELECT account_number, user_id, balance, created_at FROM ACCOUNTS;")
            .fetch_all(pool)
            .await?;
    Ok(res)
}
pub async fn get_account(
    pool: &SqlitePool,
    id: i64,
) -> Result<models::account::AccountGeneral, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_account`");
    let account: models::account::AccountGeneral = sqlx::query_as(
        "SELECT account_number, user_id, balance, created_at FROM ACCOUNTS WHERE id = ?;",
    )
    .bind(&id)
    .fetch_one(pool)
    .await?;
    Ok(account)
}
pub async fn get_account_by_account_number(
    pool: &SqlitePool,
    account_number: String,
) -> Result<models::account::AccountGeneral, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_account_by_account_number`");
    let account: models::account::AccountGeneral = sqlx::query_as(
        "SELECT account_number, user_id, balance, created_at FROM ACCOUNTS WHERE account_number = ?;",
    )
    .bind(&account_number)
    .fetch_one(pool)
    .await?;
    Ok(account)
}
pub async fn create_account(
    pool: &SqlitePool,
    account_creation: models::account::AccountCreation,
) -> Result<models::account::AccountGeneral, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_account`");
    let user_id = account_creation.user_id;
    let account_number = generation_service::generate_numeric_string(20); // TODO: MAKE ENV
    let res =
        sqlx::query("INSERT INTO ACCOUNTS (account_number, user_id, balance) VALUES (?, ?, ?);")
            .bind(account_number)
            .bind(user_id.to_string())
            .bind(String::from_str("0")?)
            .execute(pool)
            .await?;
    let created = get_account(pool, res.last_insert_rowid()).await?;
    Ok(created)
}

#[cfg(test)]
mod tests {
    use crate::queries;

    use super::*;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::inmemory:").await.unwrap();
        sqlx::query(queries::CREATE_TABLE_USER)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(queries::CREATE_TABLE_ACCOUNT)
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_get_accounts_empty() {
        let db = setup_db().await;
        let accounts = get_accounts(&db).await.unwrap();
        assert!(accounts.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_get_account() {
        let db = setup_db().await;
        let _ = user_service::create_user(
            &db,
            models::user::UserCreation {
                username: "test_user".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let account_creation = models::account::AccountCreation { user_id: 1 };
        let _ = create_account(&db, account_creation.clone()).await.unwrap();

        let accounts = get_accounts(&db).await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].user_id, 1);
    }

    #[tokio::test]
    async fn test_multiple_accounts() {
        let db = setup_db().await;
        let _ = user_service::create_user(
            &db,
            models::user::UserCreation {
                username: "a".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let _ = user_service::create_user(
            &db,
            models::user::UserCreation {
                username: "b".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let _ = user_service::create_user(
            &db,
            models::user::UserCreation {
                username: "c".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let users = vec![1, 2, 3];
        for user in &users {
            let account_creation = models::account::AccountCreation { user_id: *user };
            let res = create_account(&db, account_creation).await;
            assert!(res.is_ok())
        }
        let accounts = get_accounts(&db).await.unwrap();
        assert_eq!(accounts.len(), users.len());
        for (i, account) in accounts.iter().enumerate() {
            assert_eq!(account.user_id, users[i]);
        }
    }
    #[tokio::test]
    async fn test_create_account_with_duplicate_user_id() {
        let db = setup_db().await;
        let _ = user_service::create_user(
            &db,
            models::user::UserCreation {
                username: "c".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let account_creation = models::account::AccountCreation { user_id: 1 };
        let _ = create_account(&db, account_creation.clone()).await.unwrap();

        // Try to create another account with the same user_id
        let result = create_account(&db, account_creation.clone()).await;
        // Should succeed because account_number is unique, not user_id
        assert!(result.is_ok());

        let accounts = get_accounts(&db).await.unwrap();
        // There should be two accounts with the same user_id
        let count = accounts.iter().filter(|a| a.user_id == 1).count();
        assert_eq!(count, 2);
    }
}
