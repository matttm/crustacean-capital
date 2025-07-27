
// src/models/transaction.rs
// Defines the Transaction struct
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i32>, // AUTO_INCREMENT
    pub account_number: String,
    pub seller: String,
    pub amount: f32, // DECIMAL type
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
