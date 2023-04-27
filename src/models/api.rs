use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
// TODO: change to datetime
pub struct CreateDocumentRequest {
    pub name: String,
}