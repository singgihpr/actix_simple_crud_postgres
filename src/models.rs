use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)] // Struct untuk request POST (tanpa id)
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser { // Struct khusus untuk update, tanpa id
    pub name: Option<String>,   // Gunakan Option agar field bisa di-update sebagian
    pub email: Option<String>,  // Gunakan Option agar field bisa di-update sebagian
}