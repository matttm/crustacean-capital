use sqlx::SqlitePool;

use crate::enums;
use crate::models;
use crate::models::transaction::{TransactionCreation, TransactionGeneral};
use std::{str::FromStr, sync::Arc};

pub async fn get_transactions(
    db: &SqlitePool,
) -> Result<Vec<models::transaction::TransactionGeneral>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_transactions`");
    let transactions: Vec<models::transaction::TransactionGeneral> =
        sqlx::query_as("SELECT id, account_number, seller, amount FROM TRANSACTIONS;")
            .fetch_all(db)
            .await?;
    Ok(transactions)
}
pub async fn create_transaction(
    tx: &sqlx::Transaction
    transaction_creation: models::transaction::TransactionCreation,
) -> Result<models::transaction::TransactionCreation, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_transaction`");
    // get account and checck balance
    let account_number = transaction_creation.account_number.to_string();
    let mut balance: f32 =
        sqlx::query_scalar("SELECT balance FROM ACCOUNTS WHERE account_number = ?;")
            .bind(&account_number)
            .fetch_one(&mut *tx)
            .await?;
    if transaction_creation.amount > balance {
        return enums::errors::Error::DataError("Insufficient Funds");
    }
    sqlx::query("INSERT INTO TRANSACTIONS (account_number, seller, amount) VALUES (?, ?, ?);")
        .bind(&account_number)
        .bind(&transaction_creation.seller)
        .bind(&transaction_creation.amount.to_string())
        .execute(&mut *tx)
        .await?;
    balance -= transaction_creation.amount;
    sqlx::query("UPDATE ACCOUNTS SET balance = ? WHERE account_number = ?;")
        .bind(balance.to_string())
        .bind(account_number)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(transaction_creation)
}

#[cfg(test)]
mod tests {
    use crate::{
        queries,
        services::{account_service, user_service},
    };

    use super::*;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(queries::CREATE_TABLE_USER)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(queries::CREATE_TABLE_ACCOUNT)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(queries::CREATE_TABLE_TRANSACTION)
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_get_transactions_empty() {
        let db = setup_db().await;
        let result = get_transactions(&db).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_create_transaction_success() {
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
        let account = account_service::create_account(&db, account_creation.clone())
            .await
            .unwrap();
        let anumber = account.account_number.clone();
        let tx = TransactionCreation {
            account_number: anumber.clone(),
            seller: "TestSeller".to_string(),
            amount: -50.0,
        };
        let result = create_transaction(&db, tx.clone()).await.unwrap();
        assert_eq!(result.account_number, anumber.clone());
        assert_eq!(result.seller, "TestSeller");
        assert_eq!(result.amount, -50.0);

        let transactions = get_transactions(&db).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].account_number, anumber.clone());
        assert_eq!(transactions[0].seller, "TestSeller");
        assert_eq!(transactions[0].amount, -50.0);
    }

    #[tokio::test]
    async fn test_create_transaction_insufficient_funds() {
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
        let account = account_service::create_account(&db, account_creation.clone())
            .await
            .unwrap();
        let anumber = account.account_number.clone();
        let tx = TransactionCreation {
            account_number: anumber.clone(),
            seller: "TestSeller".to_string(),
            amount: 200.0,
        };
        let result = create_transaction(&db, tx).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Insufficient funds");
    }

    #[tokio::test]
    async fn test_get_transactions_multiple() {
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
        let account = account_service::create_account(&db, account_creation.clone())
            .await
            .unwrap();
        let anumber = account.account_number.clone();
        let tx1 = TransactionCreation {
            account_number: anumber.clone(),
            seller: "Employer1".to_string(),
            amount: -100.0,
        };
        let tx2 = TransactionCreation {
            account_number: anumber.clone(),
            seller: "Seller2".to_string(),
            amount: 20.0,
        };
        create_transaction(&db, tx1).await.unwrap();
        create_transaction(&db, tx2).await.unwrap();

        let transactions = get_transactions(&db).await.unwrap();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].seller, "Employer1");
        assert_eq!(transactions[1].seller, "Seller2");
    }

    #[tokio::test]
    async fn test_create_transaction_calculate_balance() {
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
        let account = account_service::create_account(&db, account_creation.clone())
            .await
            .unwrap();
        let anumber = account.account_number.clone();
        let tx = TransactionCreation {
            account_number: anumber.clone(),
            seller: "SellerC".to_string(),
            amount: -100.0,
        };
        let _ = create_transaction(&db, tx.clone()).await.unwrap();
        let result = create_transaction(
            &db,
            TransactionCreation {
                amount: 0f32,
                ..tx.clone()
            },
        )
        .await
        .unwrap();
        assert_eq!(result.amount, 0.0);

        let account = account_service::get_account_by_account_number(&db, anumber.clone())
            .await
            .unwrap();
        assert_eq!(account.balance, 100.0);
    }

    #[tokio::test]
    async fn test_create_transaction_negative_amount() {
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
        let account = account_service::create_account(&db, account_creation.clone())
            .await
            .unwrap();
        let anumber = account.account_number.clone();
        let tx = TransactionCreation {
            account_number: anumber.clone(),
            seller: "SellerD".to_string(),
            amount: -10.0,
        };
        let result = create_transaction(&db, tx).await;
        assert!(result.is_ok());
    }
}
