use crate::models;
use crate::services;
use axum::{Json, extract::State};
use sqlx::SqlitePool;

#[axum::debug_handler]
pub async fn get_accounts(
    State(db): State<SqlitePool>,
) -> Json<Vec<models::account::AccountGeneral>> {
    tracing::info!("Invocation to `get_accounts`");
    let res = services::account_service::get_accounts(&db).await;
    Json(res.unwrap())
}
#[axum::debug_handler]
pub async fn create_account(
    State(db): State<SqlitePool>,
    account: Json<models::account::AccountCreation>,
) -> Json<models::account::AccountGeneral> {
    tracing::info!("Invocation to `create_accounts`");
    let res = services::account_service::create_account(&db, account.0).await;
    Json(res.unwrap())
}
