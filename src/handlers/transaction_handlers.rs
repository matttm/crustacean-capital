use crate::models;
use crate::services;
use axum::{Json, extract::State};
use sqlx::SqlitePool;

#[axum::debug_handler]
pub async fn get_transactions(
    State(db): State<SqlitePool>,
) -> Json<Vec<models::transaction::TransactionGeneral>> {
    tracing::info!("Invocation to `get_transactions`");
    let res = services::transaction_service::get_transactions(&db).await;
    Json(res.unwrap())
}
#[axum::debug_handler]
pub async fn create_transaction(
    State(db): State<SqlitePool>,
    transaction: Json<models::transaction::TransactionCreation>,
) -> Json<models::transaction::TransactionCreation> {
    let mut tx = db.begin().await?;
    tracing::info!("Invocation to `create_transactions`");
    let res = services::transaction_service::create_transaction(&tx, transaction.0).await;
    Json(res.unwrap())
}
