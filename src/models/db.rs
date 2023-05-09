use chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{self, types::{chrono::Utc, Json}};
use uuid::Uuid;
use serde_json::{json, Value};

#[derive(sqlx::FromRow, Serialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub content: Value,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize)]
#[derive(Debug)]
pub struct GetDocumentNamesResponse {
    pub id: Uuid,
    pub name: String,
    pub modified_at: DateTime<Utc>,
}

