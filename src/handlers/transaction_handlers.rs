use crate::models;
use crate::services;
use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<rusqlite::Connection>>;

#[axum::debug_handler]
pub async fn get_transactions(
    State(db): State<Db>,
) -> Json<Vec<models::transaction::TransactionGeneral>> {
    tracing::info!("Invocation to `get_transactions`");
    let res = services::transaction_service::get_transactions(db).await;
    Json(res.unwrap())
}
#[axum::debug_handler]
pub async fn create_transaction(
    State(db): State<Db>,
    transaction: Json<models::transaction::TransactionCreation>,
) -> Json<models::transaction::TransactionCreation> {
    tracing::info!("Invocation to `create_transactions`");
    let res = services::transaction_service::create_transaction(db, transaction.0).await;
    Json(res.unwrap())
}
