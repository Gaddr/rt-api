use std::any::Any;

use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct ModifyDocumentMetadataRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateDocumentRequest {
    pub id: Uuid,
    pub content: Value,
}