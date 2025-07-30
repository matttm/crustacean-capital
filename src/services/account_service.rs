use crate::models;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::services::generation_service;

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
pub async fn create_account(
    db: Db,
    account_creation: models::account::AccountCreation,
) -> Result<models::account::AccountCreation, Box<dyn std::error::Error>> {
    tracing::info!("Invocation to `create_account`");
    let conn = db.lock().unwrap();
    let user_id = account_creation.user_id;
    let account_number = generation_service::generate_numeric_string(20); // TODO: MAKE ENV
    conn.execute(
        "INSERT INTO ACCOUNTS (account_number, user_id, balance) VALUES (?, ?, ?);",
        [account_number, user_id.to_string(), String::from_str("0")?],
    )?;
    Ok(account_creation)
}
