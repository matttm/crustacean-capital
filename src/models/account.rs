// src/models/account.rs
// Defines the Account struct
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i32>, // AUTO_INCREMENT
    pub account_number: String,
    pub user_id: i32, // Foreign key, assuming it's always present
    pub balance: i32, // INT, so i32
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
