// src/models/user.rs
// Defines the User struct
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>, // AUTO_INCREMENT, so it might be None before insertion
    pub username: String,
    // pub password: String,  // hidden from frontend
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserCreation {
    pub username: String,
    pub password: String, // hidden from frontend
}
