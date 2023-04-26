use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericClaims {
    pub user_id: Uuid,
    pub name: String,
    pub iat: u64,
    pub exp: u64,
    pub role: i32,
    pub org: String,
}
