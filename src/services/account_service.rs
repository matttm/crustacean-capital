use crate::models;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::services::{generation_service, user_service};
use rusqlite::Connection;

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_accounts(
    db: Db,
) -> Result<Vec<models::account::AccountGeneral>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_accounts`");
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT account_number, user_id, created_at FROM ACCOUNTS;")?;
    let mut raw = stmt.query(())?;
    let mut res: Vec<models::account::AccountGeneral> = vec![];
    while let Some(row) = raw.next()? {
        res.push(models::account::AccountGeneral {
            account_number: row.get(0)?,
            user_id: row.get(1)?,
            created_at: row.get(2)?,
        })
    }
    Ok(res)
}
pub async fn get_account(
    db: Db,
    id: i64,
) -> Result<models::account::AccountGeneral, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_accounts`");
    let conn = db.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT account_number, user_id, created_at FROM ACCOUNTS WHERE id = ?;")?;
    let account = stmt.query_one([&id], |row| {
        Ok(models::account::AccountGeneral {
            account_number: row.get(0)?,
            user_id: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;
    Ok(account)
}
pub async fn create_account(
    db: Db,
    account_creation: models::account::AccountCreation,
) -> Result<models::account::AccountGeneral, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_account`");
    let conn = db.lock().unwrap();
    let user_id = account_creation.user_id;
    let account_number = generation_service::generate_numeric_string(20); // TODO: MAKE ENV
    conn.execute(
        "INSERT INTO ACCOUNTS (account_number, user_id, balance) VALUES (?, ?, ?);",
        [account_number, user_id.to_string(), String::from_str("0")?],
    )?;
    let created = get_account(db.clone(), conn.last_insert_rowid()).await?;
    Ok(created)
}

#[cfg(test)]
mod tests {
    use crate::queries;

    use super::*;

    fn setup_db() -> Db {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(queries::CREATE_TABLE_USER, []).unwrap();
        conn.execute(queries::CREATE_TABLE_ACCOUNT, []).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_get_accounts_empty() {
        let db = setup_db();
        let accounts = get_accounts(db).await.unwrap();
        assert!(accounts.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_get_account() {
        let db = setup_db();
        let _ = user_service::create_user(
            db.clone(),
            models::user::UserCreation {
                username: "test_user".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let account_creation = models::account::AccountCreation { user_id: 1 };
        let _ = create_account(db.clone(), account_creation.clone())
            .await
            .unwrap();

        let accounts = get_accounts(db.clone()).await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].user_id, 1);
    }

    #[tokio::test]
    async fn test_multiple_accounts() {
        let db = setup_db();
        let _ = user_service::create_user(
            db.clone(),
            models::user::UserCreation {
                username: "a".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let _ = user_service::create_user(
            db.clone(),
            models::user::UserCreation {
                username: "b".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let _ = user_service::create_user(
            db.clone(),
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
            let res = create_account(db.clone(), account_creation).await;
            assert!(res.is_ok())
        }
        let accounts = get_accounts(db.clone()).await.unwrap();
        assert_eq!(accounts.len(), users.len());
        for (i, account) in accounts.iter().enumerate() {
            assert_eq!(account.user_id, users[i]);
        }
    }
    #[tokio::test]
    async fn test_create_account_with_duplicate_user_id() {
        let db = setup_db();
        let _ = user_service::create_user(
            db.clone(),
            models::user::UserCreation {
                username: "c".to_string(),
                password: "password".to_string(),
            },
        )
        .await
        .unwrap();
        let account_creation = models::account::AccountCreation { user_id: 1 };
        let _ = create_account(db.clone(), account_creation.clone())
            .await
            .unwrap();

        // Try to create another account with the same user_id
        let result = create_account(db.clone(), account_creation.clone()).await;
        // Should succeed because account_number is unique, not user_id
        assert!(result.is_ok());

        let accounts = get_accounts(db.clone()).await.unwrap();
        // There should be two accounts with the same user_id
        let count = accounts.iter().filter(|a| a.user_id == 1).count();
        assert_eq!(count, 2);
    }
}
