use chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
