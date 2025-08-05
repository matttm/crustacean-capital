// src/models/account.rs
// Defines the Account struct
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Option<i32>, // AUTO_INCREMENT
    pub account_number: String,
    pub user_id: i32, // Foreign key, assuming it's always present
    pub balance: i32, // INT, so i32
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountGeneral {
    pub account_number: String,
    pub user_id: i32, // Foreign key, assuming it's always present
    pub created_at: NaiveDateTime,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountCreation {
    pub user_id: i32, // Foreign key, assuming it's always present
}
