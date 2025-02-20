use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Uuid};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance_cents: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
