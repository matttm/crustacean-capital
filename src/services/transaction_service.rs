use crate::models;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

type Db = Arc<Mutex<rusqlite::Connection>>;

pub async fn get_transactions(
    db: Db,
) -> Result<Vec<models::transaction::TransactionGeneral>, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `get_transactions`");
    let conn = db.lock().unwrap();
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
    let mut conn = db.lock().unwrap();
    let tx = conn.transaction()?;
    let account_number = transaction_creation.account_number.to_string();
    tx.execute(
        "INSERT INTO TRANSACTIONS (account_number, seller, amount) VALUES (?, ?, ?);",
        [
            account_number,
            transaction_creation.seller.to_string(),
            transaction_creation.amount.to_string(),
        ],
    )?;
    tx.commit()?;
    Ok(transaction_creation)
}
