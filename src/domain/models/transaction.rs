use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Uuid};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub id: Uuid,
    pub source_account_id: Uuid,
    pub destination_account_id: Uuid,
    pub amount_cents: i64,
    pub created_at: Option<NaiveDateTime>,
}
