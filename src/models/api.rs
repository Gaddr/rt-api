use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct GetDocumentsResponse {
    pub id: Uuid,
    pub name: String,
    pub modified_at: DateTime<Utc>,
}