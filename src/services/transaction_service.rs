use crate::models;
use crate::models::transaction::{TransactionCreation, TransactionGeneral};
use rusqlite::Connection;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_transactions(
    db: Db,
) -> Result<Vec<models::transaction::TransactionGeneral>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_transactions`");
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id, account_number, seller, amount FROM TRANSACTIONS;")?;
    let mut raw = stmt.query(())?;
    let mut res: Vec<models::transaction::TransactionGeneral> = vec![];
    while let Some(row) = raw.next()? {
        res.push(models::transaction::TransactionGeneral {
            id: row.get(0)?,
            account_number: row.get(1)?,
            seller: row.get(2)?,
            amount: row.get(3)?,
        })
    }
    Ok(res)
}
pub async fn create_transaction(
    db: Db,
    transaction_creation: models::transaction::TransactionCreation,
) -> Result<models::transaction::TransactionCreation, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_transaction`");
    let mut conn = db.lock().await;
    let tx = conn.transaction()?;
    // get account and checck balance
    let account_number = transaction_creation.account_number.to_string();
    let mut balance: f32 = tx.query_one(
        "SELECT id, balance FROM ACCOUNTS;",
        [&account_number],
        |row| row.get(1),
    )?;
    if transaction_creation.amount > balance {
        return Err("Insufficient funds".into());
    }
    tx.execute(
        "INSERT INTO TRANSACTIONS (account_number, seller, amount) VALUES (?, ?, ?);",
        [
            &account_number,
            &transaction_creation.seller,
            &transaction_creation.amount.to_string(),
        ],
    )?;
    balance -= transaction_creation.amount;
    tx.execute(
        "UPDATE ACCOUNTS SET balance WHERE account_number = ?;",
        [balance.to_string(), account_number],
    )?;
    tx.commit()?;
    Ok(transaction_creation)
}

#[cfg(test)]
mod tests {
    use crate::{
        queries,
        services::{account_service, user_service},
    };

    use super::*;

    fn setup_db() -> Db {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(queries::CREATE_TABLE_USER, []).unwrap();
        conn.execute(queries::CREATE_TABLE_ACCOUNT, []).unwrap();
        conn.execute(queries::CREATE_TABLE_TRANSACTION, []).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_get_transactions_empty() {
        let db = setup_db();
        let result = get_transactions(db).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_create_transaction_success() {
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
        let _ = account_service::create_account(db.clone(), account_creation.clone())
            .await
            .unwrap();
        let tx = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "TestSeller".to_string(),
            amount: 50.0,
        };
        let result = create_transaction(db.clone(), tx.clone()).await.unwrap();
        assert_eq!(result.account_number, "12345");
        assert_eq!(result.seller, "TestSeller");
        assert_eq!(result.amount, 50.0);

        let transactions = get_transactions(db).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].account_number, "12345");
        assert_eq!(transactions[0].seller, "TestSeller");
        assert_eq!(transactions[0].amount, 50.0);
    }

    #[tokio::test]
    async fn test_create_transaction_insufficient_funds() {
        let db = setup_db();
        let tx = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "TestSeller".to_string(),
            amount: 200.0,
        };
        let result = create_transaction(db, tx).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Insufficient funds");
    }

    #[tokio::test]
    async fn test_get_transactions_multiple() {
        let db = setup_db();
        let tx1 = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "Seller1".to_string(),
            amount: 10.0,
        };
        let tx2 = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "Seller2".to_string(),
            amount: 20.0,
        };
        create_transaction(db.clone(), tx1).await.unwrap();
        create_transaction(db.clone(), tx2).await.unwrap();

        let transactions = get_transactions(db).await.unwrap();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].seller, "Seller1");
        assert_eq!(transactions[1].seller, "Seller2");
    }
    async fn test_create_transaction_updates_balance() {
        let db = setup_db();
        let tx = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "SellerA".to_string(),
            amount: 40.0,
        };
        create_transaction(db.clone(), tx).await.unwrap();

        let conn = db.lock().await;
        let mut stmt = conn
            .prepare("SELECT balance FROM ACCOUNTS WHERE account_number = '12345';")
            .unwrap();
        let balance: f32 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(balance, 60.0);
    }

    #[tokio::test]
    async fn test_create_transaction_invalid_account() {
        let db = setup_db();
        let tx = TransactionCreation {
            account_number: "99999".to_string(),
            seller: "SellerB".to_string(),
            amount: 10.0,
        };
        let result = create_transaction(db, tx).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_transaction_zero_amount() {
        let db = setup_db();
        let tx = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "SellerC".to_string(),
            amount: 0.0,
        };
        let result = create_transaction(db.clone(), tx.clone()).await.unwrap();
        assert_eq!(result.amount, 0.0);

        let conn = db.lock().await;
        let mut stmt = conn
            .prepare("SELECT balance FROM ACCOUNTS WHERE account_number = '12345';")
            .unwrap();
        let balance: f32 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(balance, 100.0);
    }

    #[tokio::test]
    async fn test_create_transaction_negative_amount() {
        let db = setup_db();
        let tx = TransactionCreation {
            account_number: "12345".to_string(),
            seller: "SellerD".to_string(),
            amount: -10.0,
        };
        let result = create_transaction(db, tx).await;
        assert!(result.is_err());
    }
}
