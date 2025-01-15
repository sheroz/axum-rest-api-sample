use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub id: Uuid,
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount_cents: u64,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionResult {
    Success(Transaction),
    InsufficientFunds,
    SourceAccountNotFound(Uuid),
    DestinationAccountNotFound(Uuid),
}
